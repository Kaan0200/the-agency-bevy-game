use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    winit::WinitSettings,
};

const BLU: Color = Color::rgb(56.0, 134.0, 151.0);
const _LIME: Color = Color::rgb(84.0, 201.0, 68.0);

const SAND: Color = Color::rgb(255.0, 232.0, 130.0);
const SUN: Color = Color::rgb(255.0, 253.0, 119.0);
const GOLD: Color = Color::rgb(250.0, 162.0, 52.0);

const DARK: Color = Color::rgb(40.0, 45.0, 50.0);
const FLAT: Color = Color::rgb(50.0, 50.0, 50.0);
const LIFT: Color = Color::rgb(80.0, 80.0, 100.0);

#[derive(Resource, Deref, DerefMut)]
struct InViewTab(u8);

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins((DefaultPlugins, BuildAgenciesPlugin))
        // Only run the app when there is user input. [Huge Speedup]
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(InViewTab(0))
        .add_systems(Startup, setup)
        .add_systems(Update, tab_system)
        .run();
}

#[derive(Component)]
struct Agency;

#[derive(Component)]
struct Agent;

#[derive(Component)]
struct Name(String);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // ui camera
    commands.spawn(Camera2dBundle::default());

    let tab_container = NodeBundle {
        style: Style {
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    };

    let _tile_floor = MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(RegularPolygon::new(30.0, 4))),
        material: materials.add(SAND),
        transform: Transform::with_scale(Transform::from_xyz(0., 0., 0.), Vec3::new(1., 0.3, 1.)),
        ..default()
    };

    let agents_tab = ButtonBundle {
        style: Style {
            width: Val::Px(100.),
            height: Val::Percent(33.),
            border: UiRect::new(Val::Px(2.), Val::Px(0.), Val::Px(2.), Val::Px(2.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(BLU.into()),
        background_color: Color::BLACK.into(),
        ..default()
    };
    let missions_tab = ButtonBundle {
        style: Style {
            width: Val::Px(100.),
            height: Val::Percent(33.),
            border: UiRect::new(Val::Px(2.), Val::Px(0.), Val::Px(2.), Val::Px(2.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(BLU.into()),
        background_color: Color::BLACK.into(),
        ..default()
    };
    let agency_tab = ButtonBundle {
        style: Style {
            width: Val::Px(100.),
            height: Val::Percent(33.),
            border: UiRect::new(Val::Px(2.), Val::Px(0.), Val::Px(2.), Val::Px(2.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(BLU.into()),
        background_color: Color::BLACK.into(),
        ..default()
    };

    commands.spawn(tab_container).with_children(|parent| {
        parent.spawn(agents_tab).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Agents",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 12.,
                    color: Color::WHITE.into(),
                },
            ));
        });
        parent.spawn(missions_tab).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Missions",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 12.,
                    color: Color::WHITE.into(),
                },
            ));
        });
        parent.spawn(agency_tab).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Agency",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 12.,
                    color: Color::WHITE.into(),
                },
            ));
        });
    });
}

/* Yet Unused */
fn create_agency(mut commands: Commands) {
    commands.spawn((Agent, Name("Human Joe".to_string())));
    commands.spawn((Agent, Name("Demon Rask".to_string())));

    commands.spawn((Agency, Name("Player".to_string())));
}

/* Yet Unused */
fn announce_agents(_time: Res<Time>, query: Query<&Name, With<Agent>>) {
    for name in &query {
        //    println!("created {}", name.0);
    }
}

fn tab_system(
    mut tab_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, _children) in &mut tab_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::PURPLE.into();
            }
            Interaction::Hovered => {
                *color = Color::BLUE.into();
            }
            Interaction::None => {
                *color = Color::BLACK.into();
            }
        }
    }
}

pub struct BuildAgenciesPlugin;

impl Plugin for BuildAgenciesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_agency);
        app.add_systems(Update, announce_agents);
    }
}
