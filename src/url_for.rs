use std::collections::HashMap;

use url::Url;

use iron::prelude::*;
use router::RouterInner;
#[cfg(feature = "mount")]
use mount::OriginalUrl;

/// Generate a URL based off of the currently requested URL.
///
/// The `route_id` used during route registration will be used here again.
///
/// `params` will be inserted as route parameters if fitting, the rest will be appended as query
/// parameters.
pub fn url_for(request: &Request, route_id: &str, params: HashMap<String, String>) -> ::iron::Url {
    let url = request.url.clone().into_generic_url();
    _url_for(url, request, route_id, params)
}

/// Generate a URL based off of the currently requested URL.
/// In contrast to `url_for` this method respects the hidden path if the router is mounted.
///
/// The `route_id` used during route registration will be used here again.
///
/// `params` will be inserted as route parameters if fitting, the rest will be appended as query
/// parameters.
#[cfg(feature = "mount")]
pub fn mounted_url_for(request: &Request, route_id: &str, params: HashMap<String, String>) -> ::iron::Url {
    let url = request.extensions.get::<OriginalUrl>().unwrap().clone().into_generic_url();
    _url_for(url, request, route_id, params)
}

fn _url_for(mut url: Url, request: &Request, route_id: &str, params: HashMap<String, String>) -> ::iron::Url {
    let inner = request.extensions.get::<RouterInner>().expect("Couldn\'t find router set up properly.");
    let glob = inner.route_ids.get(route_id).expect("No route with that ID");

    url_for_impl(&mut url, glob, params);
    ::iron::Url::from_generic_url(url).unwrap()
}

fn url_for_impl(url: &mut Url, glob: &str, mut params: HashMap<String, String>) {
    {
        let globs = glob.split('/');
        let globs_count = globs.clone().filter(|x| *x != "").count();
        let segments_count = url.path_segments().unwrap().count();
        let mut url_path_segments = url.path_segments_mut().unwrap();

        if globs_count < segments_count {
            for _ in 0..globs_count+1 {
                url_path_segments.pop();
            }
        } else {
            url_path_segments.clear();
        }

        let mut idx = 0;
        for path_segment in globs {
            if path_segment.len() > 1 && (path_segment.starts_with(':') || path_segment.starts_with('*')) {
                let key = &path_segment[1..];
                match params.remove(key) {
                    Some(x) => url_path_segments.push(&x),
                    None => panic!("No value for key {}", key)
                };
            } else {
                if idx == 0 && path_segment == "" {
                  idx += 1;
                } else {
                  url_path_segments.push(path_segment);
                }
            }
        }
    }

    // Now add on the remaining parameters that had no path match.
    url.set_query(None);
    if !params.is_empty() {
        url.query_pairs_mut()
            .extend_pairs(params.into_iter());
    }

    url.set_fragment(None);
}

#[cfg(test)]
mod test {
    use super::url_for_impl;
    use std::collections::HashMap;

    #[test]
    fn test_no_trailing_slash() {
        let mut url = "http://localhost/foo/bar/baz".parse().unwrap();
        url_for_impl(&mut url, "/foo/:user", {
            let mut rv = HashMap::new();
            rv.insert("user".into(), "bam".into());
            rv
        });
        assert_eq!(url.to_string(), "http://localhost/foo/bam");
    }

    #[test]
    fn test_trailing_slash() {
        let mut url = "http://localhost/foo/bar/baz".parse().unwrap();
        url_for_impl(&mut url, "/foo/:user/", {
            let mut rv = HashMap::new();
            rv.insert("user".into(), "bam".into());
            rv
        });
        assert_eq!(url.to_string(), "http://localhost/foo/bam/");
    }

    #[test]
    fn test_with_mount() {
        let mut url = "http://localhost/mounted/foo/bar/baz".parse().unwrap();
        url_for_impl(&mut url, "/foo/:user/", {
            let mut rv = HashMap::new();
            rv.insert("user".into(), "bam".into());
            rv
        });
        assert_eq!(url.to_string(), "http://localhost/mounted/foo/bam/");
    }
}
