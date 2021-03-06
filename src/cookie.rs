use httpd::Request;


#[derive(PartialEq, Clone, Debug)]
pub struct Cookie<'a> {
   pub name: &'a str,
   pub value: &'a str,
   pub expires: Option<i64>,
   pub max_age: Option<u64>,
   pub domain: Option<&'a str>,
   pub path: Option<&'a str>,
   pub secure: bool,
   pub httponly: bool,
}

impl<'a> Cookie<'a> {
   pub fn new(name: &'a str, value: &'a str) -> Cookie<'a> {
      Cookie::<'a> {
         name: name,
         value: value,
         expires: None,
         max_age: None,
         domain: None,
         path: Some("/"),
         secure: false,
         httponly: false,
      }
   }

   pub fn attrs(&self, r: &Request) -> String {
      let mut res = String::new();

      if self.httponly {
         res.push_str(";HttpOnly");
      }

      if self.secure {
         res.push_str(";Secure");
      }

      match self.path {
         Some(ref s) => res.push_str(format!(";Path={}", s).as_ref()),
         None => {}
      }

      match self.domain {
         Some(ref s) => res.push_str(format!(";Domain={}", s).as_ref()),
         None => {}
      }

      match self.max_age {
         Some(n) => res.push_str(format!(";Max-Age={}", n).as_ref()),
         None => {}
      }

      match self.expires {
         Some(ref t) => {
            match r.rfc822_date(*t) {
               Ok(s) => res.push_str(format!(";Expires={}", s).as_ref()),
               Err(_) => {}
            }
         },
         None => {}
      }

      if res.len() > 0 {
         res.remove(0);
      }

      res
   }
}
