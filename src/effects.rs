use bevy::{math::VectorSpace, prelude::*};
use bevy_hanabi::prelude::*;

pub fn green_steam() -> EffectAsset{
    let writer = ExprWriter::new();
    let init_pos = SetAttributeModifier::new(
        Attribute::POSITION, 
        writer.lit(Vec3::ZERO).expr()
    );
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
            (0.0, Vec4::new(0.2, 0.0, 0.9, 0.0)),
            (0.5, Vec4::new(0.2, 0.0, 1.0, 1.0)),
            (2.0, Vec4::new(0.0, 0.0, 0.0, 0.0)),
        ]),
        blend: ColorBlendMode::Overwrite,
        mask: ColorBlendMask::RGBA
    };
   
    let render_size = SizeOverLifetimeModifier {
        gradient: Gradient::from_keys([
            (0.3, Vec3::new(5.0, 5.0, 5.0)),
            (1.0, Vec3::new(7.0, 7.0, 7.0)),
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
