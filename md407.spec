Name: md407
Version: 1.0
Release: 1%{?dist}
Summary: Basic support for md407 computer
BuildArch: noarch

License: MIT
URL: https://github.com/Malmz/md407
Source0: https://github.com/Malmz/md407/archive/refs/tags/v%version.tar.gz

Requires: bash ripgrep fzf picocom

%description
Basic support for md407 computer

%prep
%setup -q

%build
#todo

%install
rm -rf $RPM_BUILD_ROOT
mkdir -p $RPM_BUILD_ROOT%{_bindir}
mkdir -p $RPM_BUILD_ROOT%{_sysconfdir}/udev/rules.d/
install -m 0755 md407 $RPM_BUILD_ROOT%{_bindir}/md407
install -m 0755 set_md407_baud $RPM_BUILD_ROOT%{_bindir}/set_md407_baud
install -m 0755 10-md407.rules $RPM_BUILD_ROOT%{_sysconfdir}/udev/rules.d/10-md407.rules

%clean
rm -rf $RPM_BUILD_ROOT

%files
%license LICENSE
%{_bindir}/md407
%{_bindir}/set_md407_baud
%{_sysconfdir}/udev/rules.d/10-md407.rules
