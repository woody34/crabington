use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-white shadow-sm shadow-primary-200 lg:static lg:overflow-y-visible">
            <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
                <div class="relative flex justify-between lg:gap-8 xl:grid xl:grid-cols-12">
                    <div class="flex md:absolute md:inset-y-0 md:left-0 lg:static xl:col-span-2">
                    <div class="flex flex-shrink-0 items-center">
                        <a href="#">
                            <img class="block h-8 w-auto" src="https://rustacean.net/assets/rustacean-flat-gesture.png" alt="Crabington Components" />
                        </a>
                    </div>
                    </div>
                    <div class="min-w-0 flex-1 md:px-8 lg:px-0 xl:col-span-6">
                    <div class="flex items-center px-6 py-4 md:mx-auto md:max-w-3xl lg:mx-0 lg:max-w-none xl:px-0">
                        // <div class="w-full">
                        //     <label for="search" class="sr-only">{"Search"}</label>
                        //     <div class="relative">
                        //         <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
                        //         // <!-- Heroicon name: mini/magnifying-glass -->
                        //         <svg class="h-5 w-5 text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                        //             <path fill-rule="evenodd" d="M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z" clip-rule="evenodd" />
                        //         </svg>
                        //         </div>
                        //         <input id="search" name="search" class="block w-full rounded-md border border-gray-300 bg-white py-2 pl-10 pr-3 text-sm placeholder-gray-500 focus:border-rose-500 focus:text-gray-900 focus:placeholder-gray-400 focus:outline-none focus:ring-1 focus:ring-rose-500 sm:text-sm" placeholder="Search" />
                        //     </div>
                        // </div>
                    </div>
                    </div>
                    <div class="flex items-center md:absolute md:inset-y-0 md:right-0 lg:hidden">
                        // <!-- Mobile menu button -->
                        <button type="button" class="-mx-2 inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-rose-500" aria-expanded="false">
                            <span class="sr-only">{"Open menu"}</span>
                            // <!--
                            // Icon when menu is closed.

                            // Heroicon name: outline/bars-3

                            // Menu open: "hidden", Menu closed: "block"
                            // -->
                            <svg class="block h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                            </svg>
                            // <!--
                            // Icon when menu is open.

                            // Heroicon name: outline/x-mark

                            // Menu open: "block", Menu closed: "hidden"
                            // -->
                            <svg class="hidden h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                    <div class="hidden lg:flex lg:items-center lg:justify-end xl:col-span-4">
                        // <a href="#" class="text-sm font-medium text-gray-900 hover:underline">{"Go Premium"}</a>
                        // <a href="#" class="ml-5 flex-shrink-0 rounded-full bg-white p-1 text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-rose-500 focus:ring-offset-2">
                        //     <span class="sr-only">{"View notifications"}</span>
                        //     // <!-- Heroicon name: outline/bell -->
                        //     <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                        //     <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
                        //     </svg>
                        // </a>

                        // <!-- Profile dropdown -->
                        // <div class="relative ml-5 flex-shrink-0">
                        //     <div>
                        //         <button type="button" class="flex rounded-full bg-white focus:outline-none focus:ring-2 focus:ring-rose-500 focus:ring-offset-2" id="user-menu-button" aria-expanded="false" aria-haspopup="true">
                        //             <span class="sr-only">{"Open user menu"}</span>
                        //             <img class="h-8 w-8 rounded-full" src="https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80" alt="" />
                        //         </button>
                        //     </div>

                        //     // <!--
                        //     // Dropdown menu, show/hide based on menu state.

                        //     // Entering: "transition ease-out duration-100"
                        //     //     From: "transform opacity-0 scale-95"
                        //     //     To: "transform opacity-100 scale-100"
                        //     // Leaving: "transition ease-in duration-75"
                        //     //     From: "transform opacity-100 scale-100"
                        //     //     To: "transform opacity-0 scale-95"
                        //     // -->
                        //     <div class="absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="user-menu-button" tabindex="-1">
                        //     // <!-- Active: "bg-gray-100", Not Active: "" -->
                        //     <a href="#" class="block py-2 px-4 text-sm text-gray-700" role="menuitem" tabindex="-1" id="user-menu-item-0">{"Your Profile"}</a>

                        //     <a href="#" class="block py-2 px-4 text-sm text-gray-700" role="menuitem" tabindex="-1" id="user-menu-item-1">{"Settings"}</a>

                        //     <a href="#" class="block py-2 px-4 text-sm text-gray-700" role="menuitem" tabindex="-1" id="user-menu-item-2">{"Sign out"}</a>
                        //     </div>
                        // </div>

                        // <a href="#" class="ml-6 inline-flex items-center rounded-md border border-transparent bg-rose-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-rose-700 focus:outline-none focus:ring-2 focus:ring-rose-500 focus:ring-offset-2">{"New Post"}</a>
                    </div>
                </div>
            </div>

            // <!-- Mobile menu, show/hide based on menu state. -->
            <nav class="lg:hidden" aria-label="Global">
            <div class="mx-auto max-w-3xl space-y-1 px-2 pt-2 pb-3 sm:px-4">
                // <!-- Current: "bg-gray-100 text-gray-900", Default: "hover:bg-gray-50" -->
                <a href="#" aria-current="page" class="bg-gray-100 text-gray-900 block rounded-md py-2 px-3 text-base font-medium">{"Home"}</a>

                <a href="#" class="hover:bg-gray-50 block rounded-md py-2 px-3 text-base font-medium">{"Popular"}</a>

                <a href="#" class="hover:bg-gray-50 block rounded-md py-2 px-3 text-base font-medium">{"Communities"}</a>

                <a href="#" class="hover:bg-gray-50 block rounded-md py-2 px-3 text-base font-medium">{"Trending"}</a>
            </div>
            // <div class="border-t border-gray-200 pt-4">
            //     <div class="mx-auto flex max-w-3xl items-center px-4 sm:px-6">
            //     <div class="flex-shrink-0">
            //         <img class="h-10 w-10 rounded-full" src="https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80" alt="" />
            //     </div>
            //     <div class="ml-3">
            //         <div class="text-base font-medium text-gray-800">{"Chelsea Hagon"}</div>
            //         <div class="text-sm font-medium text-gray-500">{"chelsea.hagon@example.com"}</div>
            //     </div>
            //     <button type="button" class="ml-auto flex-shrink-0 rounded-full bg-white p-1 text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-rose-500 focus:ring-offset-2">
            //         <span class="sr-only">{"View notifications"}</span>
            //         // <!-- Heroicon name: outline/bell -->
            //         <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
            //         <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
            //         </svg>
            //     </button>
            //     </div>
            //     <div class="mx-auto mt-3 max-w-3xl space-y-1 px-2 sm:px-4">
            //     <a href="#" class="block rounded-md py-2 px-3 text-base font-medium text-gray-500 hover:bg-gray-50 hover:text-gray-900">{"Your Profile"}</a>

            //     <a href="#" class="block rounded-md py-2 px-3 text-base font-medium text-gray-500 hover:bg-gray-50 hover:text-gray-900">{"Settings"}</a>

            //     <a href="#" class="block rounded-md py-2 px-3 text-base font-medium text-gray-500 hover:bg-gray-50 hover:text-gray-900">{"Sign out"}</a>
            //     </div>
            // </div>

            // <div class="mx-auto mt-6 max-w-3xl px-4 sm:px-6">
            //     <a href="#" class="flex w-full items-center justify-center rounded-md border border-transparent bg-rose-600 px-4 py-2 text-base font-medium text-white shadow-sm hover:bg-rose-700">{"New Post"}</a>

            //     <div class="mt-6 flex justify-center">
            //     <a href="#" class="text-base font-medium text-gray-900 hover:underline">{"Go Premium"}</a>
            //     </div>
            // </div>
            </nav>
        </header>
    }
}
