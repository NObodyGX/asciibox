gnome = import('gnome')

res = run_command('sh', 'build.sh', 'prepare')
if res.returncode() == 0
  message('resource generate finished.')
else
  message('resource generate failed.')
endif

gnome.compile_resources(app_name,
  app_name + '.gresource.xml',
  gresource_bundle: true,
  install: false,
  install_dir: pkg_dir,
)

# desktop file
desktop_file = i18n.merge_file(
  input: app_id + '.desktop.in',
  output: app_id + '.desktop',
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
  input: configure_file(
    input: f'@app_id@.metainfo.xml.in.in',
    output: f'@app_id@.metainfo.xml.in',
    configuration: conf,
  ),
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
