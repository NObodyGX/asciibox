pkgdatadir = join_paths(get_option('prefix'), get_option('datadir'), meson.project_name())
gnome = import('gnome')

gnome.compile_resources('asciibox',
  'asciibox.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)

install_data('com.github.nobodygx.asciibox.gschema.xml',
  install_dir: join_paths(get_option('datadir'), 'glib-2.0/schemas')
)
# Compile schemas locally, so we can run uninstalled under a devenv
gnome.compile_schemas()

compile_schemas = find_program('glib-compile-schemas', required: false)
if compile_schemas.found()
  test('Validate schema file', compile_schemas,
    args: ['--strict', '--dry-run', meson.current_source_dir()]
  )
endif

subdir('icons')


