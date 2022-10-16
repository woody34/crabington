use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MoleculeIconProps {
    #[prop_or_default]
    pub classes: String
}

#[function_component(MoleculeIcon)]
pub fn molecule_icon(MoleculeIconProps { classes }: &MoleculeIconProps) -> Html {
    let classes = classes.clone();
    html! {
        <svg class={classes} id="Layer_1" data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
            <path class="cls-1" d="M448,128a64,64,0,1,0-53.39-28.71L337,156.9a128,128,0,0,0-158-3.16L123.91,94a38.41,38.41,0,1,0-18.46,17.74l54.77,59.33A128,128,0,0,0,156.89,337L77.2,416.7a51.21,51.21,0,1,0,18.1,18.1L175,355.11A128,128,0,0,0,333.28,358l46,49.55A51.1,51.1,0,1,0,398,390.15l-46-49.54A128,128,0,0,0,355.1,175l57.61-57.61A63.69,63.69,0,0,0,448,128ZM89.6,89.6a12.8,12.8,0,1,1,12.8-12.8A12.81,12.81,0,0,1,89.6,89.6ZM51.2,486.4a25.6,25.6,0,1,1,25.6-25.6A25.63,25.63,0,0,1,51.2,486.4Zm371.2-76.8a25.6,25.6,0,1,1-25.6,25.6A25.63,25.63,0,0,1,422.4,409.6ZM358.4,256A102.4,102.4,0,1,1,256,153.6,102.52,102.52,0,0,1,358.4,256ZM448,25.6A38.4,38.4,0,1,1,409.6,64,38.44,38.44,0,0,1,448,25.6Z" />
        </svg>
    }
}
