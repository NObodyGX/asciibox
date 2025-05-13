
conf_file = 'config.rs'
configure_file(
  input: conf_file + '.in',
  output: conf_file,
  configuration: conf
)

# copy output
run_command(
  'cp',
  join_paths(meson.project_build_root(), 'src', conf_file),
  join_paths(meson.project_source_root(), 'src', conf_file),
  check: true
)


rust_sources = files([
  'main.rs',
  ]
)
subdir('core')
subdir('gui')

sources = [cargo_sources, rust_sources]

cargo_script = find_program(join_paths(meson.project_source_root(), 'data/scripts/cargo.sh'))
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
