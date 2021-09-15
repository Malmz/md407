Name: md407
Version: 1
Release: 1%{?dist}
Summary: Basic support for md407 computer
License: MIT

Requires: bash ripgrep fzf

%description
Basic support for md407 computer

%prep
#todo

%build
#todo

%install
rm -rf $RPM_BUILD_ROOT
install -m 0755 ./src/md407 $RPM_BUILD_ROOT/${_bindir}/md407
install -m 0755 src/set_md407_baud $RPM_BUILD_ROOT/${_bindir}/set_md407_baud
install -m 0755 src/10-md407.rules $RPM_BUILD_ROOT/${_sysconfdir}/udev/rules.d/10-md407.rules

%clean
rm -rf $RPM_BUILD_ROOT

%files
%{_bindir}/md407
%{_bindir}/set_md407_baud
${_sysconfdir}/udev/rules.d/10-md407.rules
