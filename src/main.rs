use bevy::prelude::*;
use bevy_rapier3d::prelude::*;


fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "RevoluteJointTest".to_string(),
            width: 360.0,
            height: 640.0,
            ..default()
        })
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .run();
}


fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec3::new(0.0, 0.0, -1.0);

    commands
    .spawn_bundle(Camera3dBundle{
        transform: Transform::from_xyz(0.1, -0.9, 0.3).looking_at(Vec3::new(0.1, -0.35, 0.0), Vec3::Z),
        ..default()
    });

    //Floor
    let floor = commands.spawn()
    .insert(RigidBody::Fixed)
    .with_children(|children| {
        children.spawn()
        .insert(Collider::cuboid(0.4, 0.7, 0.01));
    })
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(0.0, 0.0, -0.05),
            ..default()
        }
    ))
    .insert_bundle(TransformBundle::from(
        Transform{
            rotation: Quat::from_rotation_x(0.1),
            ..default()
        }
    ))
    .id();

    //OneWayGate
    let gate_anchor_pos = Vec3::new(0.1, -0.43, 0.09);
    
    let gate_anchor =
    commands.spawn()
    .insert(RigidBody::Fixed)
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(gate_anchor_pos.x, gate_anchor_pos.y, gate_anchor_pos.z),
            ..default()
        }
    ))
    .id();

    let x = Vec3::new(1.0, 0.0, 0.0);
    let joint = RevoluteJointBuilder::new(x)
        .limits([0.0, std::f32::consts::PI / 2.0])
        .local_anchor1(Vec3::new(0.0, 0.0, 0.0))
        .local_anchor2(Vec3::new(-0.03, 0.0, 0.04));

    let gate = 
    commands.spawn()
    .insert(RigidBody::Dynamic)
    .insert(Sleeping::disabled())
    .insert(Ccd::enabled())
    .insert(Collider::cuboid(0.03,0.003, 0.04))
    .with_children(|children| {
        children.spawn()
        .insert(ImpulseJoint::new(gate_anchor, joint));
    })
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(0.1, -0.43, 0.05),
            ..default()
        }
    ))
    .id();   

    //Bumper
    let bumper_pos = Vec3::new(0.14, -0.1, 0.01);
    
    let bumper =
    commands.spawn()
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.03, 0.01, 0.02))
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(bumper_pos.x, bumper_pos.y, bumper_pos.z),
            ..default()
        }
    ))
    .id();

    /* 
    let launcher_joint_pos_right = Vec3::new(0.16, -0.43, 0.09);
    let launcher_joint_right =
    commands.spawn()
    .insert(RigidBody::Fixed)
    //.insert(Collider::ball(0.01))
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(launcher_joint_pos_right.x, launcher_joint_pos_right.y, launcher_joint_pos_right.z),
            ..default()
        }
    ))
    .id();

    let joint_left = SphericalJointBuilder::new().local_anchor2(Vec3::new(-0.03, 0.0, 0.04));
    let joint_right = SphericalJointBuilder::new().local_anchor2(Vec3::new(0.03, 0.0, 0.04));
    

    let launcher_gate = 
    commands.spawn()
    .insert(RigidBody::Dynamic)
    .insert(Sleeping::disabled())
    .insert(Ccd::enabled())
    .insert(Collider::cuboid(0.03,0.003, 0.04))
    .with_children(|children| {
        children.spawn()
        .insert(ImpulseJoint::new(launcher_joint_left, joint_left));

        children.spawn()
        .insert(ImpulseJoint::new(launcher_joint_right, joint_right));
    })
    .insert_bundle(TransformBundle::from(
        Transform{
            translation: Vec3::new(0.1, -0.43, 0.05),
            ..default()
        }
    ))
    .id();    
    */

    //Ball
    let ball_pos = Vec3::new(0.13, -0.5, 0.01);

    commands.spawn()
    .insert(RigidBody::Dynamic)
    .insert(Sleeping::disabled())
    .insert(Ccd::enabled())
    .insert(Collider::ball(0.015))
    .insert(ExternalImpulse{
        impulse: Vec3::new(0.0, 0.00003, 0.0),
        ..default()
    })  
    .insert_bundle(TransformBundle::from(Transform::from_xyz(ball_pos.x, ball_pos.y, ball_pos.z)));
    

    //commands.entity(floor).push_children(&[launcher_gate, launcher_joint_left, launcher_joint_right]);
    //commands.entity(floor).push_children(&[launcher_gate, launcher_joint_left, launcher_joint_right]);
    commands.entity(floor).push_children(&[gate, gate_anchor, bumper]);
}
