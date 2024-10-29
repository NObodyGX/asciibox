gnome = import('gnome')

gnome.compile_resources('asciibox',
  'asciibox.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkg_dir,
)

# desktop file
desktop_file = i18n.merge_file(
  input: f'@app_id@.desktop.in',
  output: f'@app_id@.desktop',
  type: 'desktop',
  po_dir: podir,
  install: true,
  install_dir: join_paths(get_option('datadir'), 'applications')
)
# desktop file validate
desktop_utils = find_program('desktop-file-validate', required: false)
if desktop_utils.found()
  test('Validate desktop file', desktop_utils,
    args: [desktop_file]
  )
endif

# appdata
appstream_file = i18n.merge_file(
  input: f'@app_id@.metainfo.xml.in',
  output: f'@app_id@.metainfo.xml',
  po_dir: podir,
  install: true,
  install_dir: join_paths(get_option('datadir'), 'metainfo')
)
# appdata validate
appstreamcli = find_program('appstreamcli', required: false, disabler: true)
test('Validate appstream file', appstreamcli,
     args: ['validate', '--no-net', '--explain', appstream_file])

# gschema
install_data(f'@app_id@.gschema.xml',
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
