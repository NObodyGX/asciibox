configure_file(
  input: 'config.rs.in',
  output: 'config.rs',
  configuration: conf
)

# copy output
run_command(
  'cp',
  join_paths(meson.project_build_root(), 'src', 'config.rs'),
  join_paths(meson.project_source_root(), 'src', 'config.rs'),
  check: true
)


rust_sources = files([
  'main.rs',
  'application.rs',
  'config.rs'
  ]
)

subdir('gui')
subdir('core')

sources = [cargo_sources, rust_sources]

cargo_script = find_program(join_paths(meson.project_source_root(), 'data/script/cargo.sh'))
cargo_release = custom_target(
  'cargo-build',
  build_by_default: true,
  input: sources,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    cargo_script,
    meson.project_build_root(),
    meson.project_source_root(),
    '@OUTPUT@',
    get_option('buildtype'),
    meson.project_name(),
  ]
)
