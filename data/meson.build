pkgdatadir = join_paths(get_option('prefix'), get_option('datadir'), meson.project_name())
gnome = import('gnome')


# need sudo 
#run_command(
#  'cp',
#  join_paths(meson.project_build_root(), 'data', 'com.github.nobodygx.asciibox.gschema.xml'),
#  join_paths('/usr/share/glib-2.0/schemas/', 'com.github.nobodygx.asciibox.gschema.xml'),
#  check: true
#)

gnome.compile_resources('asciibox',
  'asciibox.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)

