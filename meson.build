project('asciibox', 'rust',
          version: '0.1.0',
    meson_version: '>= 0.59.0',
  default_options: [ 'warning_level=2',
                     'werror=false',
                     'buildtype=release'
                   ],
)

gnome = import('gnome')
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