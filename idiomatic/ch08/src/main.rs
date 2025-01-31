use std::{collections::HashMap, marker::PhantomData};

use uuid::Uuid;

fn main() {
    let session = Session::new();
    println!("{:?}", session);
    if let Ok(mut session) = session.authenticate("username", "password")
    {
        session.update_property("key", "value");
        println!("{:?}", session);
        let session = session.logout();
        println!("{:?}", session);
    }
}

fn dna_to_rna(dna: &str) -> String {
    //dna.chars().map(|c| if c == 'T' { 'U' } else { c }).collect()
    dna.chars().map(|c| match c {
        'T' => 'U',
        _ => c
    }).collect()
}

pub trait SessionState {}

#[derive(Debug, Default)]
pub struct Session<State: SessionState = Initial> {
    session_id: Uuid,
    props: HashMap<String, String>,
    phantom: PhantomData<State>,
}

#[derive(Debug, Default)]
pub struct Initial;

#[derive(Debug, Default)]
pub struct Anonymous;

#[derive(Debug, Default)]
pub struct Authenticated;

#[derive(Debug, Default)]
pub struct LoggedOut;

impl SessionState for Initial {}
impl SessionState for Anonymous {}
impl SessionState for Authenticated {}
impl SessionState for LoggedOut {}

#[derive(Debug)]
pub enum ResumeResult {
    Invalid,
    Anonymous(Session<Anonymous>),
    Authenticated(Session<Authenticated>),
}

impl Session<Initial> {
    /// Returns a new session, defaulting to the anonymous state
    pub fn new() -> Session<Anonymous> {
        Session::<Anonymous> {
            session_id: Uuid::new_v4(),
            props: HashMap::new(),
            phantom: PhantomData,
        }
    }

    /// Returns the result of resuming this session from an existing ID.
    pub fn resume_from(session_id: Uuid)
        -> ResumeResult {
            ResumeResult::Authenticated(
                Session::<Authenticated> {
                    session_id,
                    props: HashMap::new(),
                    phantom: PhantomData,
                })
    }
}

impl Session<Anonymous> {
    pub fn authenticate(
        self,
        username: &str,
        password: &str,
    ) -> Result<Session<Authenticated>, Session<Anonymous>> {
        // ...
        if !username.is_empty() && !password.is_empty() {
            Ok(Session::<Authenticated> {
                session_id: self.session_id,
                props: HashMap::new(),
                phantom: PhantomData,
            })
        } else {
            Err(self)
        }
    }
}

impl Session<Authenticated> {
    pub fn update_property(&mut self, key: &str, value: &str) {
        if let Some(prop) = self.props.get_mut(key) {
            *prop = value.to_string();
        } else {
            self.props.insert(key.to_string(), value.to_string());
        }
        // ...
    }
    
    pub fn logout(self) -> Session<LoggedOut> {
            // ...
        Session {
            session_id: Uuid::nil(),
            props: HashMap::new(),
            phantom: PhantomData,
        }
    }
}        

#[cfg(test)]
mod tests {
    use super::dna_to_rna;

    #[test]
    fn returns_expected() {
      assert_eq!(dna_to_rna("TTTT"), "UUUU");
      assert_eq!(dna_to_rna("GCAT"), "GCAU");
    }
}