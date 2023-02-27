use bevy::{
    asset::load_internal_asset,
    core_pipeline::core_3d::Transparent3d,
    ecs::system::{
        lifetimeless::{Read, SQuery, SRes},
        SystemParamItem,
    },
    math::vec3,
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_phase::*,
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
        texture::BevyDefault,
        view::{ViewUniform, ViewUniformOffset, ViewUniforms},
        Extract, RenderApp, RenderStage,
    }, core::cast_slice,
};
//use bytemuck::cast_slice;

use crate::components::*;

// these values are not used, but need to set the vertex buffer to something
const AABB_VERTEX_POSITIONS: [Vec3; 8] = [
    vec3(0.5, 0.5, -0.5),
    vec3(-0.5, 0.5, -0.5),
    vec3(-0.5, 0.5, 0.5),
    vec3(0.5, 0.5, 0.5),
    vec3(0.5, -0.5, -0.5),
    vec3(-0.5, -0.5, -0.5),
    vec3(-0.5, -0.5, 0.5),
    vec3(0.5, -0.5, 0.5),
];

const AABB_INDICES: [u32; 24] = [
    0, 1, 1, 2, 2, 3, 3, 0, // Top ring
    4, 5, 5, 6, 6, 7, 7, 4, // Bottom ring
    0, 4, 1, 5, 2, 6, 3, 7, // Verticals
];

const AABB_INDICES_LEN: u32 = AABB_INDICES.len() as u32;

const AABBS_SHADER: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 16671319754544664048);

pub struct DebugEntityAabbPlugin;

impl Plugin for DebugEntityAabbPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, AABBS_SHADER, "entity_aabb.wgsl", Shader::from_wgsl);

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<AabbsPipeline>()
                .init_resource::<AabbsMeta>()
                .init_resource::<Aabbs>()
                .add_render_command::<Transparent3d, DrawBvhAabb>()
                .add_system_to_stage(RenderStage::Extract, extract_aabbs)
                .add_system_to_stage(RenderStage::Prepare, prepare_aabbs)
                .add_system_to_stage(RenderStage::Queue, queue_aabbs);
        }
    }
}

#[derive(Resource, Clone, Debug, Default, Deref, DerefMut)]
struct Aabbs( pub Vec<Aabb>);

#[derive(Clone, Copy, Debug, Default, ShaderType)]
struct GpuAabb {
    mins: Vec3,
    maxs: Vec3,
}

#[derive(Resource)]
struct AabbsMeta {
    vertex_buffer: Buffer,
    index_buffer: Option<Buffer>,
    index_count: u32,
    instances: StorageBuffer<GpuAabbsArray>,
    bind_group: Option<BindGroup>,
    view_bind_group: Option<BindGroup>,
}

impl FromWorld for AabbsMeta {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let mut instances = StorageBuffer::<GpuAabbsArray>::default();
        instances.set_label(Some("aabbs_array"));

        let vertex_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            usage: BufferUsages::VERTEX,
            label: Some("aabb_vertex_buffer"),
            contents: cast_slice(&AABB_VERTEX_POSITIONS),
        });

        Self {
            vertex_buffer,
            index_buffer: None,
            index_count: 0,
            instances,
            bind_group: None,
            view_bind_group: None,
        }
    }
}

#[derive(Default, ShaderType)]
struct GpuAabbsArray {
    #[size(runtime)]
    array: Vec<GpuAabb>,
}

fn extract_aabbs(query: Extract<Query<&Aabb>>, mut aabbs: ResMut<Aabbs>) {
    aabbs.clear();

    for aabb in query.iter() {
        aabbs.push(*aabb)
    }
}

fn prepare_aabbs(
    aabbs: Res<Aabbs>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mut aabb_meta: ResMut<AabbsMeta>,
) {
    let instances = aabb_meta.instances.get_mut();
    instances.array.clear();

    for aabb in aabbs.iter() {
        instances.array.push(GpuAabb {
            mins: aabb.mins,
            maxs: aabb.maxs,
        });
    }
    let instance_count = instances.array.len();
    aabb_meta.index_count = instance_count as u32 * AABB_INDICES_LEN;

    if aabb_meta.index_count > 0 {
        let mut indices = Vec::with_capacity(aabb_meta.index_count as usize);
        for i in 0..instance_count {
            for index in AABB_INDICES.iter() {
                indices.push(i as u32 * AABB_INDICES_LEN + index);
            }
        }
        aabb_meta.index_buffer = Some(render_device.create_buffer_with_data(
            &BufferInitDescriptor {
                label: Some("gpu_quads_index_buffer"),
                contents: cast_slice(&indices),
                usage: BufferUsages::INDEX,
            },
        ));

        aabb_meta
            .instances
            .write_buffer(&render_device, &render_queue);
    }
}

fn queue_aabbs(
    draw_functions: Res<DrawFunctions<Transparent3d>>,
    aabb_pipeline: Res<AabbsPipeline>,
    render_device: Res<RenderDevice>,
    mut aabb_meta: ResMut<AabbsMeta>,
    view_uniforms: Res<ViewUniforms>,
    mut views: Query<&mut RenderPhase<Transparent3d>>,
) {
    if aabb_meta.index_count > 0 {
        let draw_aabb_function = draw_functions.read().get_id::<DrawBvhAabb>().unwrap();

        if let Some(view_binding) = view_uniforms.uniforms.binding() {
            aabb_meta.view_bind_group =
                Some(render_device.create_bind_group(&BindGroupDescriptor {
                    entries: &[BindGroupEntry {
                        binding: 0,
                        resource: view_binding,
                    }],
                    label: Some("aabb_view_bind_group"),
                    layout: &aabb_pipeline.view_layout,
                }));
        }

        if aabb_meta.is_changed() {
            aabb_meta.bind_group = Some(render_device.create_bind_group(&BindGroupDescriptor {
                label: Some("aabbs_bind_group"),
                layout: &aabb_pipeline.aabbs_layout,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: aabb_meta.instances.buffer().unwrap().as_entire_binding(),
                }],
            }));
        }

        for mut transparent3d_phase in views.iter_mut() {
            transparent3d_phase.add(Transparent3d {
                draw_function: draw_aabb_function,
                pipeline: aabb_pipeline.pipeline_id,
                entity: Entity::from_raw(0),
                distance: 0.0,
            });
        }
    }
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AabbPipelineKey;

#[derive(Resource)]
struct AabbsPipeline {
    pub pipeline_id: CachedRenderPipelineId,
    view_layout: BindGroupLayout,
    aabbs_layout: BindGroupLayout,
}

impl FromWorld for AabbsPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let view_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                // View
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: Some(ViewUniform::min_size()),
                    },
                    count: None,
                },
            ],
            label: Some("aabb_view_layout"),
        });

        let aabbs_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            // aabb
            label: Some("aabb_layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(0),
                },
                count: None,
            }],
        });

        let vertex_format = vec![
            // Position
            VertexFormat::Float32x4,
            // Uv
            //VertexFormat::Float32x4,
            // Color
            //VertexFormat::Float32x4,
        ];

        let vertex_layout =
            VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, vertex_format);

        // TODO: should be 1 default, but fails, using 4 for now
        let sample_count = world.get_resource::<Msaa>().map(|m| m.samples).unwrap_or(4);

        let pipeline_descriptor = RenderPipelineDescriptor {
            vertex: VertexState {
                shader: AABBS_SHADER.typed(),
                entry_point: "vertex".into(),
                shader_defs: vec![],
                buffers: vec![vertex_layout],
            },
            fragment: Some(FragmentState {
                shader: AABBS_SHADER.typed(),
                shader_defs: vec![],
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            layout: Some(vec![view_layout.clone(), aabbs_layout.clone()]),
            primitive: PrimitiveState {
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Line,
                conservative: false,
                topology: PrimitiveTopology::LineList,
                strip_index_format: None,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_compare: CompareFunction::Always,
                stencil: StencilState {
                    front: StencilFaceState::IGNORE,
                    back: StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: DepthBiasState {
                    constant: 0,
                    slope_scale: 0.0,
                    clamp: 0.0,
                },
                depth_write_enabled: true,
            }),
            multisample: MultisampleState {
                count: sample_count,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("aabb_pipeline".into()),
        };

        let mut pipeline_cache = world.resource_mut::<PipelineCache>();
        let pipeline_id = pipeline_cache.queue_render_pipeline(pipeline_descriptor);

        Self {
            pipeline_id,
            view_layout,
            aabbs_layout,
        }
    }
}

type DrawBvhAabb = (
    SetAabbPipeline,
    SetAabbViewBindGroup<0>,
    SetAabbBindGroup<1>,
    DrawAabbInstance,
);

struct SetAabbPipeline;
impl<P: PhaseItem> RenderCommand<P> for SetAabbPipeline {
    type Param = (SRes<PipelineCache>, SRes<AabbsPipeline>);
    #[inline]
    fn render<'w>(
        _view: Entity,
        _item: &P,
        params: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let (pipeline_cache, aabbs_pipeline) = params;
        if let Some(pipeline) = pipeline_cache
            .into_inner()
            .get_render_pipeline(aabbs_pipeline.pipeline_id)
        {
            pass.set_render_pipeline(pipeline);
            RenderCommandResult::Success
        } else {
            RenderCommandResult::Failure
        }
    }
}

struct SetAabbViewBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetAabbViewBindGroup<I> {
    type Param = (SRes<AabbsMeta>, SQuery<Read<ViewUniformOffset>>);

    #[inline]
    fn render<'w>(
        view: Entity,
        _item: Entity,
        (aabb_meta, view_query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let view_uniform = view_query.get(view).unwrap();
        pass.set_bind_group(
            I,
            aabb_meta.into_inner().view_bind_group.as_ref().unwrap(),
            &[view_uniform.offset],
        );
        RenderCommandResult::Success
    }
}

struct SetAabbBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetAabbBindGroup<I> {
    type Param = SRes<AabbsMeta>;

    #[inline]
    fn render<'w>(
        _view: Entity,
        _item: Entity,
        aabb_meta: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let aabb_meta = aabb_meta.into_inner();
        pass.set_bind_group(I, aabb_meta.bind_group.as_ref().unwrap(), &[]);
        RenderCommandResult::Success
    }
}

struct DrawAabbInstance;
impl EntityRenderCommand for DrawAabbInstance {
    type Param = SRes<AabbsMeta>;

    #[inline]
    fn render<'w>(
        _view: Entity,
        _item: Entity,
        aabbs_meta: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let aabbs_meta = aabbs_meta.into_inner();

        pass.set_vertex_buffer(0, aabbs_meta.vertex_buffer.slice(..));

        pass.set_index_buffer(
            aabbs_meta.index_buffer.as_ref().unwrap().slice(..),
            0,
            IndexFormat::Uint32,
        );
        pass.draw_indexed(0..aabbs_meta.index_count, 0, 0..1);

        RenderCommandResult::Success
    }
}
