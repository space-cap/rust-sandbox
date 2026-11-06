use bevy::prelude::*;
use crate::components::{
    Boundary, Collider2d, GameState, LaserBullet, Player, Speed, Velocity, WeaponCooldown,
};

/// 플레이어 우주선 조작 및 레이저 발사 시스템
pub fn player_control_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut query: Query<(&Speed, &Boundary, &mut WeaponCooldown, &mut Transform), With<Player>>,
) {
    // R 키 입력 시 게임 재시작 (Game Over 복구)
    if keyboard.just_pressed(KeyCode::KeyR) && game_state.is_game_over {
        game_state.is_game_over = false;
        game_state.score = 0;
        game_state.enemies_killed = 0;
        game_state.shields = 3;
    }

    if game_state.is_game_over {
        return;
    }

    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    for (speed, boundary, mut cooldown, mut transform) in query.iter_mut() {
        // 1. 이동 및 뱅크 턴(좌우 회전 롤) 애니메이션
        let delta = direction * speed.0 * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
        boundary.clamp(&mut transform.translation);

        let target_roll = -direction.x * 0.25;
        transform.rotation = Quat::from_rotation_z(target_roll);

        // 2. 무기 발사 쿨다운 틱
        cooldown.0.tick(time.delta());

        // 3. 스페이스바 입력 시 트윈 레이저(Twin Lasers) 발사
        if keyboard.pressed(KeyCode::Space) && cooldown.0.is_finished() {
            let px = transform.translation.x;
            let py = transform.translation.y + 18.0;

            // 좌측 레이저
            commands.spawn((
                LaserBullet,
                Velocity(Vec2::new(0.0, 700.0)),
                Collider2d::new(6.0, 18.0),
                Sprite::from_color(
                    Color::srgb(0.3, 1.0, 0.4), // 형광 라임 그린
                    Vec2::new(4.0, 16.0),
                ),
                Transform::from_xyz(px - 10.0, py, 1.5),
            ));

            // 우측 레이저
            commands.spawn((
                LaserBullet,
                Velocity(Vec2::new(0.0, 700.0)),
                Collider2d::new(6.0, 18.0),
                Sprite::from_color(
                    Color::srgb(0.3, 1.0, 0.4),
                    Vec2::new(4.0, 16.0),
                ),
                Transform::from_xyz(px + 10.0, py, 1.5),
            ));

            cooldown.0.reset();
        }
    }
}
