project(
  'minikeyboard',
  'rust',
  version: '0.1.0',
  meson_version: '>= 1.0.0',
  license: 'GPL-3.0-or-later',
  default_options: [
    'warning_level=2',
    'prefix=/usr',
  ],
)

app_id = 'br.com.quantmind.MiniKeyboard'
find_program('cargo', required: true)
sh = find_program('bash', required: true)
cargo_script = meson.project_source_root() / 'build-aux' / 'cargo.sh'
prefix = get_option('prefix')
bindir = get_option('bindir')
datadir = get_option('datadir')
udevdir = get_option('udevdir')

cargo_home = meson.project_build_root() / 'cargo-home'
cargo_target = meson.project_build_root() / 'target'
manifest = meson.project_source_root() / 'Cargo.toml'

if get_option('profile') == 'release'
  cargo_profile_args = ['--release']
  rust_target = 'release'
else
  cargo_profile_args = []
  rust_target = 'debug'
endif

cargo_env = environment()
cargo_env.set('CARGO_HOME', cargo_home)

custom_target(
  'minikeyboard-bin',
  build_by_default: true,
  build_always_stale: true,
  output: 'minikeyboard',
  console: true,
  env: cargo_env,
  command: [
    sh,
    cargo_script,
    '@OUTPUT@',
    'minikeyboard',
    rust_target,
    '--manifest-path', manifest,
    '--target-dir', cargo_target,
  ] + cargo_profile_args,
  install: true,
  install_dir: bindir,
)

custom_target(
  'minikeyboard-daemon-bin',
  build_by_default: true,
  build_always_stale: true,
  output: 'minikeyboard-daemon',
  console: true,
  env: cargo_env,
  command: [
    sh,
    cargo_script,
    '@OUTPUT@',
    'minikeyboard-daemon',
    rust_target,
    '--manifest-path', manifest,
    '--target-dir', cargo_target,
  ] + cargo_profile_args,
  install: true,
  install_dir: bindir,
)

daemon_conf = configuration_data()
daemon_conf.set('bindir', prefix / bindir)
configure_file(
  input: 'data' / 'minikeyboard-daemon.service.in',
  output: 'minikeyboard-daemon.service',
  configuration: daemon_conf,
  install: true,
  install_dir: 'lib' / 'systemd' / 'user',
)

install_data(
  'data' / app_id + '.desktop',
  install_dir: datadir / 'applications',
)

install_data(
  'data' / app_id + '.metainfo.xml',
  install_dir: datadir / 'metainfo',
)

install_data(
  'data' / 'icons' / 'hicolor' / 'scalable' / 'apps' / app_id + '.svg',
  install_dir: datadir / 'icons' / 'hicolor' / 'scalable' / 'apps',
)

install_data(
  'data' / 'udev' / '70-minikeyboard.rules',
  install_dir: udevdir / 'rules.d',
)

summary({
  'prefix': prefix,
  'bindir': bindir,
  'profile': get_option('profile'),
  'udevdir': udevdir,
})
