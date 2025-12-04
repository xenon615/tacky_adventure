use bevy::{
    // prelude::*
    prelude::default,
    math::{Vec3, Vec4}
};
use bevy_hanabi::prelude::*;

#[allow(dead_code)]
pub fn lift_steam() -> EffectAsset{
    let writer = ExprWriter::new();

    // let init_pos = SetAttributeModifier::new(
    //     Attribute::POSITION, 
    //     writer.lit(Vec3::ZERO).expr()
    // );


    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        dimension: ShapeDimension::Surface,
        radius: writer.lit(2.).expr(),
        // radius: writer.lit(2.).uniform(writer.lit(1.5)).expr(),
        axis: writer.lit(Vec3::Y).expr()
    };



    let init_age = SetAttributeModifier::new(
        Attribute::AGE,
        writer.lit(0.0).expr()
    );
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        writer.lit(5.0).expr()
    );
    let init_velocity = SetAttributeModifier::new(
        Attribute::VELOCITY,
        ((writer.rand(VectorType::VEC3F) * writer.lit(2.0) - writer.lit(1.0)).normalized() * writer.lit(0.41)).expr()
    );
    let render_color = ColorOverLifetimeModifier {
        gradient:  Gradient::from_keys([
            (0.0, Vec4::new(1.0, 0.0, 0.9, 0.0)),
            (0.5, Vec4::new(0.1, 0.0, 1.0, 1.0)),
            (2.0, Vec4::new(0.0, 0.0, 0.0, 0.0)),
        ]),
        blend: ColorBlendMode::Overwrite,
        mask: ColorBlendMask::RGBA
    };
   
    let render_size = SizeOverLifetimeModifier {
        gradient: Gradient::from_keys([
            (0.3, Vec3::new(5.0, 5.0, 5.0)),
            (1.0, Vec3::new(2.0, 2.0, 2.0)),
        ]),
        ..default()
    };    

    let render_texture = ParticleTextureModifier{
        texture_slot: writer.lit(0u32).expr(),
        sample_mapping: ImageSampleMapping::ModulateOpacityFromR,
    };

    let mut module = writer.finish();
    module.add_texture_slot("cloud");
      
    
    EffectAsset::new(
        500,
        SpawnerSettings::rate(30.0.into())
            .with_starts_active(false)
            .with_emit_on_start(false),
       module
    )
    .with_alpha_mode(bevy_hanabi::AlphaMode::Blend)
    .init(init_age)
    .init(init_lifetime)
    .init(init_pos)
    .init(init_velocity)
    .render(render_color)
    .render(render_size)
    .render(render_texture)
    .render(OrientModifier::new(OrientMode::FaceCameraPosition))
    

}

// ---

pub fn jet_stream() -> EffectAsset{
    let render_color = ColorOverLifetimeModifier::new(Gradient::from_keys(
        vec![
            (0.0, Vec4::new(2., 2., 2., 1.)),
            (0.3, Vec4::new(3., 0., 0., 0.1)),
            (0.5, Vec4::new(3., 0.0, 0.0, 0.0))
        ]
    ));
    
    let render_size = SizeOverLifetimeModifier {
        gradient: Gradient::from_keys(vec![
            (0.0, Vec3::splat(1.0)),
            (0.5, Vec3::splat(0.9)),
            (1., Vec3::splat(0.1))
        ]),
        screen_space_size:false
    };
    let writer = ExprWriter::new();

    let init_age = SetAttributeModifier::new(
        Attribute::AGE, 
        writer.lit(0.).uniform(writer.lit(0.02)).expr()
    );

    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME, 
        writer.lit(0.5).uniform(writer.lit(0.3)).expr()
    );

    let init_pos = SetPositionCone3dModifier {
        height: writer.lit(2.).expr(),
        base_radius: writer.lit(0.2).expr(),
        top_radius: writer.lit(0.1).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_velocity = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(2.) + writer.lit(2.)).expr(),
    };

    let render_texture = ParticleTextureModifier {
        texture_slot: writer.lit(0u32).expr(),
        sample_mapping: ImageSampleMapping::ModulateOpacityFromR
    };


    let mut module = writer.finish();
    module.add_texture_slot("cloud");
    EffectAsset::new(
        1000,
        SpawnerSettings::rate(200.0.into())
        .with_emit_on_start(true)
        ,
       module
    )
    .init(init_age)
    .init(init_lifetime)
    .init(init_pos)
    .init(init_velocity)
    .render(render_size)
    .render(render_color)
    .render(render_texture)

    .render(OrientModifier::new(OrientMode::FaceCameraPosition))
}

// ---

pub fn blast () -> EffectAsset {
    let render_color = ColorOverLifetimeModifier::new(
        Gradient::from_keys(
            vec![
                (0.0, Vec4::new(0., 0., 0., 1.)),
                (0.11, Vec4::new(10., 10., 0., 1.)),
                (0.2, Vec4::new(0.0, 10.0, 1.0, 1.)),
                (0.25, Vec4::new(10.0, 0.0, 0.0, 1.))
            ]
        )
    );

    let render_size = SetSizeModifier {
        size: Vec3::splat(5.).into(),
        ..default()
    };

    let writer = ExprWriter::new();

    let init_age = SetAttributeModifier::new(
        Attribute::AGE, 
        writer.lit(0.)
        .uniform(writer.lit(0.02))
        .expr()
    );

    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        // writer.lit(0.2).expr()
        // (writer.rand(ScalarType::Float) * writer.lit(0.1)).expr()
        writer.lit(0.0).uniform(writer.lit(0.1)) 
        // (writer.* writer.lit(0.1)).expr()
        .expr()
    );

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(2.).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(30.)).expr(),
    };

    let render_texture = ParticleTextureModifier {
        texture_slot: writer.lit(0u32).expr(),
        sample_mapping: ImageSampleMapping::ModulateOpacityFromR
    };

    let mut module = writer.finish();
    module.add_texture_slot("cloud");

    EffectAsset::new(
        2000,
        SpawnerSettings::once(200.0.into())
        .with_emit_on_start(false),
        module,
    )
    .init(init_pos)
    .init(init_vel)
    .init(init_age)
    .init(init_lifetime)
    .render(render_color)
    .render(render_size)
    .render(render_texture)
    .render(OrientModifier::new(OrientMode::FaceCameraPosition))
}

// ---

#[allow(dead_code)]
pub fn scattering() -> EffectAsset {

    let render_color = ColorOverLifetimeModifier::new(
        Gradient::from_keys(
            vec![
                (0.0, Vec4::new(1., 1., 0., 1.)),
                (0.5, Vec4::new(0.0, 10.0, 1.0, 1.)),
                (0.7, Vec4::new(0.0, 0.0, 10.0, 1.))
            ]
        )
    );

    let render_size = SetSizeModifier {
        size: Vec3::splat(2.).into(),
        ..default()
    };

    let writer = ExprWriter::new();

    let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.).uniform(writer.lit(0.02)).expr());

    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(0.0).uniform(writer.lit(0.8)).expr());

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(1.).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(10.)).expr(),
    };

    let render_texture = ParticleTextureModifier {
        texture_slot: writer.lit(0u32).expr(),
        sample_mapping: ImageSampleMapping::ModulateOpacityFromR
    };


    let mut module = writer.finish();
    module.add_texture_slot("cloud");

    EffectAsset::new(
        2000,
        SpawnerSettings::once(500.0.into())
        .with_emit_on_start(false),
        module,
    )
    .init(init_pos)
    .init(init_vel)
    .init(init_age)
    .init(init_lifetime)
    .render(render_color)
    .render(render_size)
    .render(render_texture)
     .render(OrientModifier::new(OrientMode::AlongVelocity))
}