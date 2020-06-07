# Introduction

Demonstration of a simple game using Rust and SDL2.

# Installation

dnf install SDL2-devel
dnf install SDL2_image-devel
dnf install SDL2_gfx-devel
dnf install SDL2_mixer-devel
dnf install SDL2_ttf-devel  

# Requirements

1. bat is special and can be controlled
2. both bat and ball are moveable
3. bat is special and can only move horizontally
4. fixed objects are special and cannot move
5. naive collision detection is O(n^2)
6. number of copies should be minimised
7. possible trade off between immutability and performance

# Sketch

Model:
    index_player: 1
    shapes: Vec<Shape>
    occupy: Vec<[bool;3]>
    moveables: Vec<usize>

init:
    for each shape:
        determine_occupancy(shape) -> Vec<index_occupy>
        occupy

update:
    update bat velocity from player movements
    for each m in moveables:
        m_new = m.update_position()
        determine_occupancy_updates(m, m_new) -> occupancy_updates
        occupy, occupancy_updates -> occupy, occupy_collisions
    for each oc in occupy_collisions:
        for each i in occupy[oc]:
            m = shape[i]
            for each j in occupy[oc] where i != j:
                o = shape[j]
                if is_intersection(m,o):
                    # what about moveable/static?
                    m_new = m.update_velocity().update_position()
                    determine_occupancy_updates(m, m_new) -> occupancy_updates
                    occupy, occupancy_updates -> occupy, _
        

 
# License

MIT
