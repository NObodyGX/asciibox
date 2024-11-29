gnome = import('gnome')

# not need build resource to target dir
gnome.compile_resources('asciibox',
  'asciibox.gresource.xml',
  gresource_bundle: true,
  install: false,
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

subdir('icons')
