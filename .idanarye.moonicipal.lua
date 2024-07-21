local moonicipal = require'moonicipal'
local T = moonicipal.tasks_file()

local P = require'idan.project.rust.bevy' {
    crate_name = 'swift_dreams_are_made_for_dweebs',
    level_editor = false,
    extra_logging = {
        -- blenvy = 'debug',
        -- bevy = 'debug',
        -- bevy_gltf_blueprints = 'debug',
        -- bevy_gltf_components = 'debug',
    },
}
moonicipal.include(P)
