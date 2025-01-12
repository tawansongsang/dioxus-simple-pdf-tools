use crate::{
    components::{NavMenu, NavMenuProps, SidebarMenu, SidebarMenuProps},
    Route,
};
use dioxus::prelude::*;

const DIP_LOGO_IMG: Asset = asset!(
    "/assets/favicon.ico",
    ImageAssetOptions::new()
        .with_size(ImageSize::Manual {
            width: 35,
            height: 30
        })
        .with_format(ImageFormat::Avif)
);
const SUN_IMG: Asset = asset!("/assets/imgs/sun-svgrepo-com.svg");
const MENU_IMG: Asset = asset!("/assets/imgs/burger-simple-svgrepo-com.svg");

#[component]
pub fn Navbar() -> Element {
    let mut is_sidebar_activate = use_signal(|| false);
    let navbar_menus = vec![
        NavMenuProps::new(Route::Merge {}, "MERGE"),
        NavMenuProps::new(Route::Split {}, "SPLIT"),
    ];
    let sidebar_menus = vec![
        SidebarMenuProps::new(Route::Home {}, "HOME"),
        SidebarMenuProps::new(Route::Merge {}, "MERGE PDF"),
        SidebarMenuProps::new(Route::Split {}, "SPLIT PDF"),
    ];

    rsx! {
        header {
            id: "header",
            class: "flex h-14 flex-row justify-between border-b-neutral-600 bg-neutral-50 shadow-md shadow-neutral-600/50",
            nav {
                id: "primary-nav",
                class: "ml-2",
                aria_labelledby: "primary-navigation",
                ul { class: "flex h-full flex-row items-center",
                    li { class: "ml-1 mr-1",
                        Link { to: Route::Home {},
                            img {
                                class: "w-7 sm:w-8 md:w-auto",
                                src: DIP_LOGO_IMG,
                            }
                        }
                    }
                    for nav_menu_props in navbar_menus {
                        NavMenu {
                            route: nav_menu_props.route,
                            name: nav_menu_props.name,
                        }
                    }
                }
            }
            nav {
                id: "sidebar",
                class: "mr-2 flex items-center",
                aria_labelledby: "sidebar-navigation",
                Link { to: Route::Home {}, class: "mr-2 sm:flex",
                    img { src: SUN_IMG, class: "h-7 sm:h-8 md:h-9" }
                }
                button { onclick: move |_| is_sidebar_activate.set(!is_sidebar_activate()),
                    img { src: MENU_IMG, class: "h-7 sm:h-8 md:h-9" }
                }
                if is_sidebar_activate() {
                    aside {
                        aria_label: "Sidebar",
                        class: "fixed left-0 top-14 z-40 h-screen w-screen overflow-y-auto  bg-neutral-700/[0.7]",
                        onclick: move |_| is_sidebar_activate.set(!is_sidebar_activate()),
                        div { class: "mt-2 flex justify-end",
                            ul { class: "flex w-52 flex-col rounded-md border-2 border-neutral-600 bg-neutral-50",
                                for sidebar_menu_props in sidebar_menus {
                                    SidebarMenu {
                                        route: sidebar_menu_props.route,
                                        name: sidebar_menu_props.name,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
