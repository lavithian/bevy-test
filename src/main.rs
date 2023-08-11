use bevy::prelude::*;
// use bevy::render::camera::ScalingMode;
use bevy::core_pipeline::clear_color::ClearColorConfig;

fn main() {
  App::new()
    .add_plugins(
      DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "Testing".into(),
            resolution: (640.0, 480.0).into(),
            resizable: false,
            ..default()
          }),
          ..default()
        })
        .build(),
    )
    .insert_resource(Money(100.0))
    .add_systems(Startup, setup)
    .add_systems(Update, (character_movement, spawn_cow, cow_lifetime))
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2dBundle {
    camera_2d: Camera2d {
      clear_color: ClearColorConfig::Custom(Color::GREEN),
    },
      ..default()
  });

  // let mut camera = Camera2dBundle::default();

  // camera.projection.scaling_mode = ScalingMode::AutoMin {
  //   min_width: 256.0,
  //   min_height: 144.0,
  // };

  // commands.spawn(camera);

  let texture = asset_server.load("char.png");

  // commands.spawn(SpriteBundle {
  //   sprite: Sprite {
  //     custom_size: Some(Vec2::new(100.0, 100.0)),
  //     ..default()
  //   },
  //   texture,
  //   ..default()
  // });
  commands.spawn((
    SpriteBundle {
      sprite: Sprite {
        custom_size: Some(Vec2::new(100.0, 100.0)),
        ..default()
      },
      texture,
      ..default()
    },
    Player { speed: 100.0 },
  ));
}

fn character_movement(
  // gain mutable access to the sprite entity
  // do by Query
  // command, query and res are most common in Bevy
  // Queries lets us get all of the component data for each entity that has all the components we list
  // we are getting mutable access to the character's transform and we only want entities that has a Sprite component
  // The query is a tuple of references. The query will match every entity that has both a transform and a Sprite component
  // gives us access to the transform and read only access to the sprite
  // without the reference to Sprite, we'd be targeting everything (e.g. camera)
  // mut characters: Query<(&mut Transform, &Sprite)>,

  // Change the query to target the Player
  mut characters: Query<(&mut Transform, &Player)>,
  // Res = resource
  input: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  // loop over all of the entities that match our query and get mutable access to the transform
  for (mut transform, player,) in &mut characters {
    let movement_amount = player.speed * time.delta_seconds();
    if input.pressed(KeyCode::W) {
      transform.translation.y += movement_amount;
    }
    if input.pressed(KeyCode::S) {
      transform.translation.y -= movement_amount;
    }
    if input.pressed(KeyCode::D) {
      transform.translation.x += movement_amount;
    }
    if input.pressed(KeyCode::A) {
      transform.translation.x -= movement_amount;
    }
  }
}

fn spawn_cow(
  mut commands: Commands, // to spawn  cow
  asset_server: Res<AssetServer>, // to load sprite
  input: Res<Input<KeyCode>>, // to press the button
  mut money: ResMut<Money>, // access to player's money
  player: Query<&Transform, With<Player>>, //the player's position Uses With filter
) {
  // reduces nesting. Instead of putting everything into big if input pressed
  if !input.just_pressed(KeyCode::Space) {
    return ;
  }
  // use .single if you know there is only one match of the query
  let player_transform = player.single();

  if money.0 >= 10.0 {
    money.0 -= 10.0;
    info!("Spent $10 on a cow! Remaining money: ${:?}", money.0);

    let texture = asset_server.load("cow.png");

    commands.spawn((
      SpriteBundle {
        sprite: Sprite {
          custom_size: Some(Vec2::new(50.0, 50.0)),
          ..default()
        },
        texture,
        transform: *player_transform,
        ..default()
      },
      Cow {
        lifetime: Timer::from_seconds(5.0, TimerMode::Once), // timer does it only once
      }
    ));
  }
}

fn cow_lifetime(
  mut commands: Commands,
  time: Res<Time>,
  mut cows: Query<(Entity, &mut Cow)>,
  mut money: ResMut<Money>,
) {
  for (cow_entity, mut cow) in &mut cows {
    cow.lifetime.tick(time.delta());

    if cow.lifetime.finished() {
      money.0 += 15.0;

      commands.entity(cow_entity).despawn();

      info!("Cow sold for $15! Current Money: ${:?}", money.0);
    }
  }
}

#[derive(Component)]
pub struct Player {
  pub speed: f32,
}

#[derive(Resource)]
pub struct Money(pub f32);

#[derive(Component)]
pub struct Cow {
  pub lifetime: Timer,
}
