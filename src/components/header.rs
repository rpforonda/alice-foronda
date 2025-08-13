use leptos::prelude::*;

/// Header section
#[component]
pub fn Header() -> impl IntoView {
    let (open, set_open) = signal(false);

    // toggle helper
    let toggle = move |_| set_open.update(|o| *o = !*o);
    let close = move |_| set_open.set(false);

    view! {
        <header class="sticky top-0 z-50 bg-white/80 backdrop-blur dark:bg-gray-900/80 border-b border-gray-200 dark:border-gray-800">
            <div class="text-4xl flex justify-center items-center px-6 py-4 lg:px-8 text-gray-900 dark:text-gray-100">
                "Alice Foronda"
            </div>
            <nav
                aria-label="Main"
                class="mx-auto max-w-7xl flex justify-center items-center px-6 py-4 lg:px-8"
            >

                <div class="hidden md:flex items-center gap-8">
                    <a
                        href="#about"
                        class="text-sm font-medium text-gray-700 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
                    >
                        "About"
                    </a>
                    <a
                        href="#books"
                        class="text-sm font-medium text-gray-700 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
                    >
                        "Books"
                    </a>
                    <a
                        href="#contact"
                        class="text-sm font-medium text-gray-700 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white"
                    >
                        "Contact"
                    </a>
                </div>

                <button
                    class="md:hidden inline-flex items-center justify-center rounded-md p-2 text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-800"
                    aria-label="Toggle menu"
                    aria-expanded=move || if open.get() { "true" } else { "false" }
                    aria-controls="mobile-menu"
                    on:click=toggle
                >
                    {move || {
                        if open.get() {
                            view! {
                                // X icon
                                <svg
                                    class="size-6"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="1.5"
                                    aria-hidden="true"
                                >
                                    <path
                                        d="M6 18 18 6M6 6l12 12"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    />
                                </svg>
                            }
                                .into_view()
                        } else {
                            view! {
                                // Hamburger
                                <svg
                                    class="size-6"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="1.5"
                                    aria-hidden="true"
                                >
                                    <path
                                        d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    />
                                </svg>
                            }
                                .into_view()
                        }
                    }}
                </button>
            </nav>

            // mobile menu view
            <Show when=move || open.get() fallback=|| ()>
                <div
                    id="mobile-menu"
                    class="md:hidden border-t border-gray-200 dark:border-gray-800"
                >
                    <div class="mx-auto max-w-7xl px-6 py-4 lg:px-8 flex flex-col gap-2">
                        <a
                            href="#about"
                            on:click=close
                            class="rounded px-3 py-2 text-base font-medium text-gray-900 hover:bg-gray-100 dark:text-white dark:hover:bg-gray-800"
                        >
                            "About"
                        </a>
                        <a
                            href="#books"
                            on:click=close
                            class="rounded px-3 py-2 text-base font-medium text-gray-900 hover:bg-gray-100 dark:text-white dark:hover:bg-gray-800"
                        >
                            "Books"
                        </a>
                        <a
                            href="#contact"
                            on:click=close
                            class="rounded px-3 py-2 text-base font-medium text-gray-900 hover:bg-gray-100 dark:text-white dark:hover:bg-gray-800"
                        >
                            "Contact"
                        </a>
                    </div>
                </div>
            </Show>
        </header>
    }
}
