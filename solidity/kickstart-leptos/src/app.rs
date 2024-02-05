use crate::{
    error_template::{AppError, ErrorTemplate},
    ethereum::{contracts::campaign::Campaign, get_factory_contract},
};
use ethers::{
    providers::{Http, Provider},
    types::Address,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::info;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/kickstart-leptos.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage ssr=SsrMode::Async />
                </Routes>
            </main>
        </Router>
    }
}

async fn fetch_campaigns() -> Result<Vec<Campaign<Provider<Http>>>, AppError> {
    let factory = get_factory_contract();
    let campaigns = factory.get_deployed_campaigns().await.unwrap();
    let campaigns: Vec<Campaign<Provider<Http>>> = campaigns
        .into_iter()
        .map(|address| Campaign::new(address, factory.client()))
        .collect();
    Ok(campaigns)
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let campaign_addresses = create_local_resource(
        || (),
        |_| async move {
            info!("fetching campaigns");
            let campaigns = fetch_campaigns()
                .await
                .unwrap()
                .iter()
                .map(|campaign| campaign.address().to_string())
                .collect::<Vec<String>>();
            info!("campaigns: {:?}", campaigns);
            campaigns
        },
    );

    view! {
        <h1>"Campaigns"</h1>
        {
            move || match campaign_addresses.get() {
                Some(campaign_addresses) => {
                    view! {
                        <ul>
                            {
                                campaign_addresses.iter().map(|address| {
                                    view! {
                                        <li>{address}</li>
                                    }
                                }).collect_view()
                            }
                        </ul>
                    }.into_view()
                }
                None => {
                    view! {
                        <p>"Loading..."</p>
                    }.into_view()
                }
            }
        }
    }
}
