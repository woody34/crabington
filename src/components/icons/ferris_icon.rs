use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FerrisIconProps {
    #[prop_or_default]
    pub classes: String
}

#[function_component(FerrisIcon)]
pub fn ferris_icon(FerrisIconProps { classes }: &FerrisIconProps) -> Html {
    let classes = classes.clone();
    html! {
        <svg class={classes} id="Layer_1" data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
            <path d="M60.5776 13.8268L51.8673 42.6431L77.7475 37.331L60.5776 13.8268Z" fill="#DEB819"/>
            <path d="M108.361 94.9937L138.708 90.686L115.342 69.8642" stroke="black" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <g filter="url(#filter0_d)">
                <circle cx="75.3326" cy="73.4918" r="55" fill="#FDD630"/>
                <circle cx="75.3326" cy="73.4918" r="52.5" stroke="black" stroke-width="5"/>
            </g>
            <circle cx="71" cy="99" r="5" fill="white" fill-opacity="0.75" stroke="black" stroke-width="3"/>
            <defs>
                <filter id="filter0_d" x="16.3326" y="18.4918" width="118" height="118" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
                    <@{"feGaussianBlur"} stdDeviation="2"/>
                    <@{"feColorMatrix"} in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"/>
                </filter>
            </defs>
        </svg>
    }
}
