project('asciibox', 'rust',
          version: '0.8.5',
    meson_version: '>= 0.59.0',
  default_options: [ 'warning_level=2',
                     'werror=false',
                     'buildtype=release'
                   ],
)

i18n = import('i18n')
gnome = import('gnome')
app_id = 'com.github.nobodygx.asciibox'
app_name = 'asciibox'
pkg_dir = join_paths(get_option('prefix'), get_option('datadir'), app_name)


conf = configuration_data()
conf.set_quoted('APP_ID', app_id)
conf.set_quoted('APP_NAME', app_name)
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GETTEXT_PACKAGE', app_name)
conf.set_quoted('PACKAGE_DATADIR', pkg_dir)
conf.set_quoted('PACKAGE_WEBSITE', 'https://github.com/nobodygx/asciibox')
conf.set_quoted('PACKAGE_BUGREPORTS', 'https://github.com/nobodygx/asciibox/issues')
conf.set_quoted('PACKAGE_ICON_NAME', app_name)
conf.set_quoted('PACKAGE_NAME', app_name)
conf.set_quoted('LOCALEDIR', join_paths(get_option('prefix'), get_option('localedir')))

dependency('gtk4', version: '>= 4.0.0')
dependency('libadwaita-1', version: '>=1.1.0')

cargo_sources = files(
  'Cargo.toml',
)

subdir('data')
subdir('src')

gnome.post_install(
  glib_compile_schemas: true,
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)
