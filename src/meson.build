pkgdatadir = join_paths(get_option('prefix'), get_option('datadir'), meson.project_name())

conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GETTEXT_PACKAGE', 'asciibox')
conf.set_quoted('LOCALEDIR', join_paths(get_option('prefix'), get_option('localedir')))
conf.set_quoted('PKGDATADIR', pkgdatadir)

rust_sources = files(
  'main.rs',
)

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
