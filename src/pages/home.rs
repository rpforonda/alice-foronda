use crate::components::footer::Footer;
use crate::components::header::Header;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <Header />
            <body class="scroll-smooth bg-white dark:bg-gray-900">
                <main class="isolate">
                    <section id="about">
                        <div class="relative isolate -z-10">
                            <svg
                                aria-hidden="true"
                                class="absolute inset-x-0 top-0 -z-10 h-256 w-full mask-[radial-gradient(32rem_32rem_at_center,white,transparent)] stroke-gray-200 dark:stroke-white/10"
                            >
                                <defs>
                                    <pattern
                                        id="1f932ae7-37de-4c0a-a8b0-a6e3b4d44b84"
                                        width="200"
                                        height="200"
                                        x="50%"
                                        y="-1"
                                        patternUnits="userSpaceOnUse"
                                    >
                                        <path d="M.5 200V.5H200" fill="none" />
                                    </pattern>
                                </defs>
                                <svg
                                    x="50%"
                                    y="-1"
                                    class="overflow-visible fill-gray-50 dark:fill-gray-800"
                                >
                                    <path
                                        d="M-200 0h201v201h-201Z M600 0h201v201h-201Z M-400 600h201v201h-201Z M200 800h201v201h-201Z"
                                        stroke-width="0"
                                    />
                                </svg>
                                <rect
                                    width="100%"
                                    height="100%"
                                    fill="url(#1f932ae7-37de-4c0a-a8b0-a6e3b4d44b84)"
                                    stroke-width="0"
                                />
                            </svg>
                            <div
                                aria-hidden="true"
                                class="absolute top-0 right-0 left-1/2 -z-10 -ml-24 transform-gpu overflow-hidden blur-3xl lg:ml-24 xl:ml-48"
                            >
                                <div
                                    style="clip-path: polygon(63.1% 29.5%, 100% 17.1%, 76.6% 3%, 48.4% 0%, 44.6% 4.7%, 54.5% 25.3%, 59.8% 49%, 55.2% 57.8%, 44.4% 57.2%, 27.8% 47.9%, 35.1% 81.5%, 0% 97.7%, 39.2% 100%, 35.2% 81.4%, 97.2% 52.8%, 63.1% 29.5%)"
                                    class="aspect-801/1036 w-200.25 bg-linear-to-tr from-[#ff80b5] to-[#9089fc] opacity-30"
                                ></div>
                            </div>
                            <div class="overflow-hidden">
                                <div class="mx-auto max-w-7xl px-6 pt-36 pb-32 sm:pt-60 lg:px-8 lg:pt-32">
                                    <div class="mx-auto max-w-2xl gap-x-14 lg:mx-0 lg:flex lg:max-w-none lg:items-center">
                                        <div class="relative w-full lg:max-w-xl lg:shrink-0 xl:max-w-2xl">
                                            <h1 class="text-5xl font-semibold tracking-tight text-pretty text-gray-900 sm:text-7xl dark:text-white">
                                                "We’re changing the way people connect"
                                            </h1>
                                            <p class="mt-8 text-lg font-medium text-pretty text-gray-500 sm:max-w-md sm:text-xl/8 lg:max-w-none dark:text-gray-400">
                                                "Cupidatat minim id magna ipsum sint dolor qui. Sunt sit in quis cupidatat mollit aute velit. Et labore commodo nulla aliqua proident mollit ullamco exercitation tempor. Sint aliqua anim nulla sunt mollit id pariatur in voluptate cillum. Eu voluptate tempor esse minim amet fugiat veniam occaecat aliqua."
                                            </p>
                                        </div>
                                        <div class="mt-14 flex justify-end gap-8 sm:-mt-44 sm:justify-start sm:pl-20 lg:mt-0 lg:pl-0">
                                            <div class="ml-auto w-44 flex-none space-y-8 pt-32 sm:ml-0 sm:pt-80 lg:order-last lg:pt-36 xl:order-0 xl:pt-80">
                                                <div class="relative">
                                                    <img
                                                        src="https://images.unsplash.com/photo-1557804506-669a67965ba0?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&h=528&q=80"
                                                        alt=""
                                                        class="aspect-2/3 w-full rounded-xl bg-gray-900/5 object-cover shadow-lg dark:bg-gray-700/5"
                                                    />
                                                    <div class="pointer-events-none absolute inset-0 rounded-xl ring-1 ring-gray-900/10 ring-inset dark:ring-white/10"></div>
                                                </div>
                                            </div>
                                            <div class="mr-auto w-44 flex-none space-y-8 sm:mr-0 sm:pt-52 lg:pt-36">
                                                <div class="relative">
                                                    <img
                                                        src="https://images.unsplash.com/photo-1485217988980-11786ced9454?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&h=528&q=80"
                                                        alt=""
                                                        class="aspect-2/3 w-full rounded-xl bg-gray-900/5 object-cover shadow-lg dark:bg-gray-700/5"
                                                    />
                                                    <div class="pointer-events-none absolute inset-0 rounded-xl ring-1 ring-gray-900/10 ring-inset dark:ring-white/10"></div>
                                                </div>
                                                <div class="relative">
                                                    <img
                                                        src="https://images.unsplash.com/photo-1559136555-9303baea8ebd?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&crop=focalpoint&fp-x=.4&w=396&h=528&q=80"
                                                        alt=""
                                                        class="aspect-2/3 w-full rounded-xl bg-gray-900/5 object-cover shadow-lg dark:bg-gray-700/5"
                                                    />
                                                    <div class="pointer-events-none absolute inset-0 rounded-xl ring-1 ring-gray-900/10 ring-inset dark:ring-white/10"></div>
                                                </div>
                                            </div>
                                            <div class="w-44 flex-none space-y-8 pt-32 sm:pt-0">
                                                <div class="relative">
                                                    <img
                                                        src="https://images.unsplash.com/photo-1670272504528-790c24957dda?ixlib=rb-4.0.3&ixid=MnwxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&crop=left&w=400&h=528&q=80"
                                                        alt=""
                                                        class="aspect-2/3 w-full rounded-xl bg-gray-900/5 object-cover shadow-lg dark:bg-gray-700/5"
                                                    />
                                                    <div class="pointer-events-none absolute inset-0 rounded-xl ring-1 ring-gray-900/10 ring-inset dark:ring-white/10"></div>
                                                </div>
                                                <div class="relative">
                                                    <img
                                                        src="https://images.unsplash.com/photo-1670272505284-8faba1c31f7d?ixlib=rb-4.0.3&ixid=MnwxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&h=528&q=80"
                                                        alt=""
                                                        class="aspect-2/3 w-full rounded-xl bg-gray-900/5 object-cover shadow-lg dark:bg-gray-700/5"
                                                    />
                                                    <div class="pointer-events-none absolute inset-0 rounded-xl ring-1 ring-gray-900/10 ring-inset dark:ring-white/10"></div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </section>

                    <section id="books">
                        <div class="mx-auto -mt-12 max-w-7xl px-6 sm:mt-0 lg:px-8 xl:-mt-8">
                            <div class="mx-auto max-w-2xl lg:mx-0 lg:max-w-none">
                                <h2 class="text-4xl font-semibold tracking-tight text-pretty text-gray-900 sm:text-5xl dark:text-white">
                                    "Our mission"
                                </h2>
                                <div class="mt-6 flex flex-col gap-x-8 gap-y-20 lg:flex-row">
                                    <div class="lg:w-full lg:max-w-2xl lg:flex-auto">
                                        <p class="text-xl/8 text-gray-600 dark:text-gray-300">
                                            "Aliquet nec orci mattis amet quisque ullamcorper neque, nibh sem. At arcu, sit dui mi, nibh dui, diam eget aliquam. Quisque id at vitae feugiat egestas ac. Diam nulla orci at in viverra scelerisque eget. Eleifend egestas fringilla sapien."
                                        </p>
                                        <p class="mt-10 max-w-xl text-base/7 text-gray-700 dark:text-gray-400">
                                            "Faucibus commodo massa rhoncus, volutpat. Dignissim sed eget risus enim. Mattis mauris semper sed amet vitae sed turpis id. Id dolor praesent donec est. Odio penatibus risus viverra tellus varius sit neque erat velit. Faucibus commodo massa rhoncus, volutpat. Dignissim sed eget risus enim. Mattis mauris semper sed amet vitae sed turpis id."
                                        </p>
                                    </div>
                                    <div class="lg:flex lg:flex-auto lg:justify-center">
                                        <dl class="w-64 space-y-8 xl:w-80">
                                            <div class="flex flex-col-reverse gap-y-4">
                                                <dt class="text-base/7 text-gray-600 dark:text-gray-400">
                                                    "Transactions every 24 hours"
                                                </dt>
                                                <dd class="text-5xl font-semibold tracking-tight text-gray-900 dark:text-white">
                                                    "44 million"
                                                </dd>
                                            </div>
                                            <div class="flex flex-col-reverse gap-y-4">
                                                <dt class="text-base/7 text-gray-600 dark:text-gray-400">
                                                    "Assets under holding"
                                                </dt>
                                                <dd class="text-5xl font-semibold tracking-tight text-gray-900 dark:text-white">
                                                    "$119 trillion"
                                                </dd>
                                            </div>
                                            <div class="flex flex-col-reverse gap-y-4">
                                                <dt class="text-base/7 text-gray-600 dark:text-gray-400">
                                                    "New users annually"
                                                </dt>
                                                <dd class="text-5xl font-semibold tracking-tight text-gray-900 dark:text-white">
                                                    "46,000"
                                                </dd>
                                            </div>
                                        </dl>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </section>

                    <div class="mt-32 sm:mt-40 xl:mx-auto xl:max-w-7xl xl:px-8">
                        <img
                            src="https://images.unsplash.com/photo-1529156069898-49953e39b3ac?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2832&q=80"
                            alt=""
                            class="aspect-5/2 w-full object-cover outline-1 -outline-offset-1 outline-black/5 xl:rounded-3xl dark:outline-white/10"
                        />
                    </div>

                    <div class="mx-auto mt-32 max-w-7xl px-6 sm:mt-40 lg:px-8">
                        <div class="mx-auto max-w-2xl lg:mx-0">
                            <h2 class="text-4xl font-semibold tracking-tight text-pretty text-gray-900 sm:text-5xl dark:text-white">
                                Our values
                            </h2>
                            <p class="mt-6 text-lg/8 text-gray-700 dark:text-gray-300">
                                Lorem ipsum dolor sit amet consect adipisicing elit. Possimus magnam voluptatum cupiditate veritatis in accusamus quisquam.
                            </p>
                        </div>
                        <dl class="mx-auto mt-16 grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 text-base/7 sm:grid-cols-2 lg:mx-0 lg:max-w-none lg:grid-cols-3">
                            <div>
                                <dt class="font-semibold text-gray-900 dark:text-white">
                                    Be world-class
                                </dt>
                                <dd class="mt-1 text-gray-600 dark:text-gray-400">
                                    Aut illo quae. Ut et harum ea animi natus. Culpa maiores et sed sint et magnam exercitationem quia. Ullam voluptas nihil vitae dicta molestiae et. Aliquid velit porro vero.
                                </dd>
                            </div>
                            <div>
                                <dt class="font-semibold text-gray-900 dark:text-white">
                                    Share everything you know
                                </dt>
                                <dd class="mt-1 text-gray-600 dark:text-gray-400">
                                    Mollitia delectus a omnis. Quae velit aliquid. Qui nulla maxime adipisci illo id molestiae. Cumque cum ut minus rerum architecto magnam consequatur. Quia quaerat minima.
                                </dd>
                            </div>
                            <div>
                                <dt class="font-semibold text-gray-900 dark:text-white">
                                    Always learning
                                </dt>
                                <dd class="mt-1 text-gray-600 dark:text-gray-400">
                                    Aut repellendus et officiis dolor possimus. Deserunt velit quasi sunt fuga error labore quia ipsum. Commodi autem voluptatem nam. Quos voluptatem totam.
                                </dd>
                            </div>
                            <div>
                                <dt class="font-semibold text-gray-900 dark:text-white">
                                    Be supportive
                                </dt>
                                <dd class="mt-1 text-gray-600 dark:text-gray-400">
                                    Magnam provident veritatis odit. Vitae eligendi repellat non. Eum fugit impedit veritatis ducimus. Non qui aspernatur laudantium modi. Praesentium rerum error deserunt harum.
                                </dd>
                            </div>
                            <div>
                                <dt class="font-semibold text-gray-900 dark:text-white">
                                    Take responsibility
                                </dt>
                                <dd class="mt-1 text-gray-600 dark:text-gray-400">
                                    Sit minus expedita quam in ullam molestiae dignissimos in harum. Tenetur dolorem iure. Non nesciunt dolorem veniam necessitatibus laboriosam voluptas perspiciatis error.
                                </dd>
                            </div>
                            <div>
                                <dt class="font-semibold text-gray-900 dark:text-white">
                                    Enjoy downtime
                                </dt>
                                <dd class="mt-1 text-gray-600 dark:text-gray-400">
                                    Ipsa in earum deserunt aut. Quos minus aut animi et soluta. Ipsum dicta ut quia eius. Possimus reprehenderit iste aspernatur ut est velit consequatur distinctio.
                                </dd>
                            </div>
                        </dl>
                    </div>

                    <div class="relative isolate -z-10 mt-32 sm:mt-48">
                        <div class="absolute inset-x-0 top-1/2 -z-10 flex -translate-y-1/2 justify-center overflow-hidden mask-[radial-gradient(50%_45%_at_50%_55%,white,transparent)]">
                            <svg
                                aria-hidden="true"
                                class="h-160 w-7xl flex-none stroke-gray-200 dark:stroke-white/10"
                            >
                                <defs>
                                    <pattern
                                        id="e9033f3e-f665-41a6-84ef-756f6778e6fe"
                                        width="200"
                                        height="200"
                                        x="50%"
                                        y="50%"
                                        patternUnits="userSpaceOnUse"
                                        patternTransform="translate(-100 0)"
                                    >
                                        <path d="M.5 200V.5H200" fill="none" />
                                    </pattern>
                                </defs>
                                <svg
                                    x="50%"
                                    y="50%"
                                    class="overflow-visible fill-gray-50 dark:fill-gray-800"
                                >
                                    <path
                                        d="M-300 0h201v201h-201Z M300 200h201v201h-201Z"
                                        stroke-width="0"
                                    />
                                </svg>
                                <rect
                                    width="100%"
                                    height="100%"
                                    fill="url(#e9033f3e-f665-41a6-84ef-756f6778e6fe)"
                                    stroke-width="0"
                                />
                            </svg>
                        </div>
                        <div class="mx-auto max-w-7xl px-6 lg:px-8">
                            <h2 class="text-center text-lg/8 font-semibold text-gray-900 dark:text-white">
                                "Trusted by the world’s most innovative teams"
                            </h2>
                            <div class="mx-auto mt-10 grid max-w-lg grid-cols-4 items-center gap-x-8 gap-y-10 sm:max-w-xl sm:grid-cols-6 sm:gap-x-10 lg:mx-0 lg:max-w-none lg:grid-cols-5">
                                <img
                                    width="158"
                                    height="48"
                                    src="https://tailwindcss.com/plus-assets/img/logos/158x48/transistor-logo-gray-900.svg"
                                    alt="Transistor"
                                    class="col-span-2 max-h-12 w-full object-contain lg:col-span-1 dark:hidden"
                                />
                                <img
                                    width="158"
                                    height="48"
                                    src="https://tailwindcss.com/plus-assets/img/logos/158x48/transistor-logo-white.svg"
                                    alt="Transistor"
                                    class="col-span-2 max-h-12 w-full object-contain not-dark:hidden lg:col-span-1"
                                />

                                <img
                                    width="158"
                                    height="48"
                                    src="https://tailwindcss.com/plus-assets/img/logos/158x48/reform-logo-gray-900.svg"
                                    alt="Reform"
                                    class="col-span-2 max-h-12 w-full object-contain lg:col-span-1 dark:hidden"
                                />
                                <img
                                    width="158"
                                    height="48"
                                    src="https://tailwindcss.com/plus-assets/img/logos/158x48/reform-logo-white.svg"
                                    alt="Reform"
                                    class="col-span-2 max-h-12 w-full object-contain not-dark:hidden lg:col-span-1"
                                />

                                <img
                                    width="158"
                                    height="48"
                                    src="https://tailwindcss.com/plus-assets/img/logos/158x48/tuple-logo-gray-900.svg"
                                    alt="Tuple"
                                    class="col-span-2 max-h-12 w-full object-contain lg:col-span-1 dark:hidden"
                                />
                                <img
                                    width="158"
                                    height="48"
                                    src="https://tailwindcss.com/plus-assets/img/logos/158x48/tuple-logo-white.svg"
                                    alt="Tuple"
                                    class="col-span-2 max-h-12 w-full object-contain not-dark:hidden lg:col-span-1"
                                />

                                <img
                                    width="158"
                                    height="48"
                                    src="https://tailwindcss.com/plus-assets/img/logos/158x48/savvycal-logo-gray-900.svg"
                                    alt="SavvyCal"
                                    class="col-span-2 max-h-12 w-full object-contain sm:col-start-2 lg:col-span-1 dark:hidden"
                                />
                                <img
                                    width="158"
                                    height="48"
                                    src="https://tailwindcss.com/plus-assets/img/logos/158x48/savvycal-logo-white.svg"
                                    alt="SavvyCal"
                                    class="col-span-2 max-h-12 w-full object-contain not-dark:hidden sm:col-start-2 lg:col-span-1"
                                />

                                <img
                                    width="158"
                                    height="48"
                                    src="https://tailwindcss.com/plus-assets/img/logos/158x48/statamic-logo-gray-900.svg"
                                    alt="Statamic"
                                    class="col-span-2 col-start-2 max-h-12 w-full object-contain sm:col-start-auto lg:col-span-1 dark:hidden"
                                />
                                <img
                                    width="158"
                                    height="48"
                                    src="https://tailwindcss.com/plus-assets/img/logos/158x48/statamic-logo-white.svg"
                                    alt="Statamic"
                                    class="col-span-2 col-start-2 max-h-12 w-full object-contain not-dark:hidden sm:col-start-auto lg:col-span-1"
                                />
                            </div>
                        </div>
                    </div>

                    <div class="mx-auto mt-32 max-w-7xl px-6 sm:mt-48 lg:px-8">
                        <div class="mx-auto max-w-2xl lg:mx-0">
                            <h2 class="text-4xl font-semibold tracking-tight text-pretty text-gray-900 sm:text-5xl dark:text-white">
                                Our team
                            </h2>
                            <p class="mt-6 text-lg/8 text-gray-600 dark:text-gray-400">
                                "We’re a dynamic group of individuals who are passionate about what we do and dedicated to delivering the best results for our clients."
                            </p>
                        </div>
                        <ul
                            role="list"
                            class="mx-auto mt-20 grid max-w-2xl grid-cols-2 gap-x-8 gap-y-16 text-center sm:grid-cols-3 md:grid-cols-4 lg:mx-0 lg:max-w-none lg:grid-cols-5 xl:grid-cols-6"
                        >
                            <li>
                                <img
                                    src="https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80"
                                    alt=""
                                    class="mx-auto size-24 rounded-full outline-1 -outline-offset-1 outline-black/5 dark:outline-white/10"
                                />
                                <h3 class="mt-6 text-base/7 font-semibold tracking-tight text-gray-900 dark:text-white">
                                    Michael Foster
                                </h3>
                                <p class="text-sm/6 text-gray-600 dark:text-gray-400">
                                    Co-Founder / CTO
                                </p>
                            </li>
                            <li>
                                <img
                                    src="https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80"
                                    alt=""
                                    class="mx-auto size-24 rounded-full outline-1 -outline-offset-1 outline-black/5 dark:outline-white/10"
                                />
                                <h3 class="mt-6 text-base/7 font-semibold tracking-tight text-gray-900 dark:text-white">
                                    Dries Vincent
                                </h3>
                                <p class="text-sm/6 text-gray-600 dark:text-gray-400">
                                    Business Relations
                                </p>
                            </li>
                            <li>
                                <img
                                    src="https://images.unsplash.com/photo-1517841905240-472988babdf9?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80"
                                    alt=""
                                    class="mx-auto size-24 rounded-full outline-1 -outline-offset-1 outline-black/5 dark:outline-white/10"
                                />
                                <h3 class="mt-6 text-base/7 font-semibold tracking-tight text-gray-900 dark:text-white">
                                    Lindsay Walton
                                </h3>
                                <p class="text-sm/6 text-gray-600 dark:text-gray-400">
                                    Front-end Developer
                                </p>
                            </li>
                            <li>
                                <img
                                    src="https://images.unsplash.com/photo-1438761681033-6461ffad8d80?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80"
                                    alt=""
                                    class="mx-auto size-24 rounded-full outline-1 -outline-offset-1 outline-black/5 dark:outline-white/10"
                                />
                                <h3 class="mt-6 text-base/7 font-semibold tracking-tight text-gray-900 dark:text-white">
                                    Courtney Henry
                                </h3>
                                <p class="text-sm/6 text-gray-600 dark:text-gray-400">Designer</p>
                            </li>
                            <li>
                                <img
                                    src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80"
                                    alt=""
                                    class="mx-auto size-24 rounded-full outline-1 -outline-offset-1 outline-black/5 dark:outline-white/10"
                                />
                                <h3 class="mt-6 text-base/7 font-semibold tracking-tight text-gray-900 dark:text-white">
                                    Tom Cook
                                </h3>
                                <p class="text-sm/6 text-gray-600 dark:text-gray-400">
                                    Director of Product
                                </p>
                            </li>
                            <li>
                                <img
                                    src="https://images.unsplash.com/photo-1517365830460-955ce3ccd263?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80"
                                    alt=""
                                    class="mx-auto size-24 rounded-full outline-1 -outline-offset-1 outline-black/5 dark:outline-white/10"
                                />
                                <h3 class="mt-6 text-base/7 font-semibold tracking-tight text-gray-900 dark:text-white">
                                    Whitney Francis
                                </h3>
                                <p class="text-sm/6 text-gray-600 dark:text-gray-400">Copywriter</p>
                            </li>
                            <li>
                                <img
                                    src="https://images.unsplash.com/photo-1519345182560-3f2917c472ef?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80"
                                    alt=""
                                    class="mx-auto size-24 rounded-full outline-1 -outline-offset-1 outline-black/5 dark:outline-white/10"
                                />
                                <h3 class="mt-6 text-base/7 font-semibold tracking-tight text-gray-900 dark:text-white">
                                    Leonard Krasner
                                </h3>
                                <p class="text-sm/6 text-gray-600 dark:text-gray-400">
                                    Senior Designer
                                </p>
                            </li>
                            <li>
                                <img
                                    src="https://images.unsplash.com/photo-1463453091185-61582044d556?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80"
                                    alt=""
                                    class="mx-auto size-24 rounded-full outline-1 -outline-offset-1 outline-black/5 dark:outline-white/10"
                                />
                                <h3 class="mt-6 text-base/7 font-semibold tracking-tight text-gray-900 dark:text-white">
                                    Floyd Miles
                                </h3>
                                <p class="text-sm/6 text-gray-600 dark:text-gray-400">
                                    Principal Designer
                                </p>
                            </li>
                            <li>
                                <img
                                    src="https://images.unsplash.com/photo-1502685104226-ee32379fefbe?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80"
                                    alt=""
                                    class="mx-auto size-24 rounded-full outline-1 -outline-offset-1 outline-black/5 dark:outline-white/10"
                                />
                                <h3 class="mt-6 text-base/7 font-semibold tracking-tight text-gray-900 dark:text-white">
                                    Emily Selman
                                </h3>
                                <p class="text-sm/6 text-gray-600 dark:text-gray-400">
                                    VP, User Experience
                                </p>
                            </li>
                            <li>
                                <img
                                    src="https://images.unsplash.com/photo-1500917293891-ef795e70e1f6?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80"
                                    alt=""
                                    class="mx-auto size-24 rounded-full outline-1 -outline-offset-1 outline-black/5 dark:outline-white/10"
                                />
                                <h3 class="mt-6 text-base/7 font-semibold tracking-tight text-gray-900 dark:text-white">
                                    Kristin Watson
                                </h3>
                                <p class="text-sm/6 text-gray-600 dark:text-gray-400">
                                    VP, Human Resources
                                </p>
                            </li>
                            <li>
                                <img
                                    src="https://images.unsplash.com/photo-1505840717430-882ce147ef2d?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80"
                                    alt=""
                                    class="mx-auto size-24 rounded-full outline-1 -outline-offset-1 outline-black/5 dark:outline-white/10"
                                />
                                <h3 class="mt-6 text-base/7 font-semibold tracking-tight text-gray-900 dark:text-white">
                                    Emma Dorsey
                                </h3>
                                <p class="text-sm/6 text-gray-600 dark:text-gray-400">
                                    Senior Developer
                                </p>
                            </li>
                            <li>
                                <img
                                    src="https://images.unsplash.com/photo-1509783236416-c9ad59bae472?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=1024&h=1024&q=80"
                                    alt=""
                                    class="mx-auto size-24 rounded-full outline-1 -outline-offset-1 outline-black/5 dark:outline-white/10"
                                />
                                <h3 class="mt-6 text-base/7 font-semibold tracking-tight text-gray-900 dark:text-white">
                                    Alicia Bell
                                </h3>
                                <p class="text-sm/6 text-gray-600 dark:text-gray-400">
                                    Junior Copywriter
                                </p>
                            </li>
                        </ul>
                    </div>

                </main>
                <Footer />
            </body>
        </ErrorBoundary>
    }
}
