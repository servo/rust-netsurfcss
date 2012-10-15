#[cfg(target_os = "macos")]
#[nolink]
#[link_args="-L../libcss -lcss -L../libparserutils -lparserutils -L../libwapcaplet -lwapcaplet -liconv"]
pub extern mod linking { }

#[cfg(target_os = "linux")]
#[nolink]
#[link_args="-L../libcss -lcss -L../libparserutils -lparserutils -L../libwapcaplet -lwapcaplet"]
pub extern mod linking { }
