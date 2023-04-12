fn get(url: &str) ->  RequestBuilder{
    return reqwest::blocking::get(url).unwrap().text(); 
}
