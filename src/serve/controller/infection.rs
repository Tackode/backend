use super::super::authorization::professional_user_filter;
use super::super::common::*;
use uuid::Uuid;
use warp::{filters::BoxedFilter, Filter, Reply};

pub fn routes(context: Context) -> BoxedFilter<(impl Reply,)> {
    let moved_context = context.clone();
    let context_filter = warp::any().map(move || moved_context.clone());

    // POST /infection -> 200
    let create_infection = warp::post()
        .and(warp::path!("infection"))
        .and(professional_user_filter(context.clone()))
        .and(warp::body::content_length_limit(CONTENT_LENGTH_LIMIT))
        .and(warp::body::json())
        .and(context_filter.clone())
        .map(create);

    // GET /infections -> Vec<Infection>
    let get_infections = warp::get()
        .and(warp::path!("infections"))
        .and(professional_user_filter(context.clone()))
        .and(context_filter.clone())
        .map(get_all);

    create_infection.or(get_infections).boxed()
}

fn create(user: ProfessionalUser, data: InfectionForm, context: Context) -> impl Reply {
    warp::reply()
}

fn get_all(user: ProfessionalUser, context: Context) -> impl Reply {
    let placeholder_id = Uuid::parse_str("85f520d0-193d-4386-bdf6-902bc7a4350e").unwrap();

    warp::reply::json(&vec![Infection {
        id: placeholder_id,
        start_timestamp: chrono::Utc::now(),
        end_timestamp: chrono::Utc::now(),
        places: vec![Place {
            id: user.id,
            organization: Organization {
                id: user.id,
                name: String::from("Creatiwity"),
            },
            name: String::from("Bureau 1"),
            description: None,
            average_duration: 600,
        }],
    }])
}
