use yew::prelude::*;
use crate::components::button::button::Button;
use crate::components::button::button::ButtonPreset;

#[function_component(Atoms)]
pub fn atoms() -> Html {
    html! {
        <div class="mt-4 content">
            <ul role="list" class="space-y-4">
                <li class="bg-white px-4 py-6 md:shadow sm:rounded-lg sm:p-6">
                    <article>
                        <div>
                            <div class="flex space-x-3">
                                <div class="min-w-0 flex-1">
                                    <p class="text-sm font-medium text-gray-900">
                                        {"Button Component"}
                                    </p>
                                </div>
                            </div>
                        </div>
                        <div class="flex flex-wrap sm:px-6 lg:px-8 mt-4 sm:-mx-6 md:mx-0 text-sm text-gray-700 grid-bg min-h-48 h-48 justify-center items-center md:rounded-lg">
                            <Button preset={ButtonPreset::Primary} label="Primary"></Button>
                            <Button preset={ButtonPreset::Secondary} label="Secondary"></Button>
                            <Button preset={ButtonPreset::Tertiary} label="Tertiary"></Button>
                        </div>
                        <div class="ring-1 ring-gray-300 px-4 sm:px-6 lg:px-8 mt-4 sm:-mx-6 md:mx-0 md:rounded-lg">
                            <table class="w-full divide-y divide-gray-300">
                                <thead>
                                    <tr>
                                        <th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6">{"Prop"}</th>
                                        <th scope="col" class="hidden lg:table-cell px-3 py-3.5 text-left text-sm font-semibold text-gray-900">{"Description"}</th>
                                        <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">{"Default Value"}</th>
                                        <th scope="col" class="hidden lg:table-cell px-3 py-3.5 text-left text-sm font-semibold text-gray-900">{"Control"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td class="py-4 pl-4 sm:pl-6 pr-3 text-sm">{"preset"}</td>
                                        <td class="hidden lg:table-cell px-3 py-3.5 text-sm text-gray-500">{"Preset styling applied to button"}</td>
                                        <td class="px-3 py-3.5 text-sm text-gray-500">{"ButtonPreset::Primary"}</td>
                                        <td class="hidden lg:table-cell px-3 py-3.5 text-sm text-gray-500">
                                            <select class="mt-1 block w-full rounded-md border-gray-300 py-2 pl-3 pr-10 text-base focus:border-primary-500 sm:text-sm">
                                                <option>{"Primary"}</option>
                                                <option>{"Secondary"}</option>
                                                <option>{"Tertiary"}</option>
                                            </select>
                                        </td>
                                    </tr>

                                    <tr>
                                        <td class="py-4 pl-4 sm:pl-6 pr-3 text-sm">{"label"}</td>
                                        <td class="hidden lg:table-cell px-3 py-3.5 text-sm text-gray-500">{"Text displayed on top of button"}</td>
                                        <td class="px-3 py-3.5 text-sm text-gray-500">{"String::new()"}</td>
                                        <td class="hidden lg:table-cell px-3 py-3.5 text-sm text-gray-500">
                                            <input type="text" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm" placeholder="Confirm" />
                                        </td>
                                    </tr>

                                    <tr>
                                        <td class="relative py-4 pl-4 sm:pl-6 pr-3 text-sm">{"class"}</td>
                                        <td class="hidden lg:table-cell px-3 py-3.5 text-sm text-gray-500">{"Class value to be appended to the button class"}</td>
                                        <td class="px-3 py-3.5 text-sm text-gray-500">{"String::new()"}</td>
                                        <td class="hidden lg:table-cell px-3 py-3.5 text-sm text-gray-500">
                                            <input type="text" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm" placeholder="hover:bg-primary-700" />
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </article>
                </li>
            </ul>
        </div>
    }
}
