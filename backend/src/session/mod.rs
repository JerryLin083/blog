use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::{sync::Mutex, time::sleep};
use uuid::Uuid;

pub fn session_builder(duration: Duration) -> Arc<SessionManager> {
    Arc::new(SessionManager {
        sessions: Arc::new(Mutex::new(HashMap::new())),
        expires: Arc::new(Mutex::new(BinaryHeap::new())),
        duration,
    })
}

pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    expires: Arc<Mutex<BinaryHeap<Expire>>>,
    duration: Duration,
}

impl SessionManager {
    pub(crate) async fn create_session(self: &mut Arc<Self>, user_id: i32) -> String {
        let mut sessions = self.sessions.lock().await;
        let mut expires = self.expires.lock().await;

        let session_id = Uuid::new_v4().to_string();
        let expired_time = Instant::now() + self.duration;

        sessions.insert(session_id.clone(), Session { user_id });
        expires.push(Expire(expired_time, session_id.clone()));

        session_id
    }

    pub(crate) async fn delete_session(self: Arc<Self>, session_id: &str) -> Option<()> {
        let mut sessions = self.sessions.lock().await;

        match sessions.remove(session_id) {
            Some(_) => Some(()),
            None => None,
        }
    }

    pub(crate) async fn check_session_id(self: Arc<Self>, session_id: &str) -> Option<Session> {
        let sessions = self.sessions.lock().await;

        sessions.get(session_id).cloned()
    }

    pub(crate) async fn check_auth(self: Arc<Self>, session_id: &str) -> Option<()> {
        let sessions = self.sessions.lock().await;

        match sessions.get(session_id) {
            Some(_) => Some(()),
            None => None,
        }
    }

    pub(crate) fn run_check(self: &Arc<Self>) {
        let session_manager = self.clone();

        tokio::spawn(async move {
            //Init next check time
            let mut check_timing = session_manager.duration;

            loop {
                sleep(check_timing).await;

                //remove expired session in expires
                let mut expires = session_manager.expires.lock().await;
                let mut session_id_vec = vec![];

                expires.retain(|expire| {
                    let expired_time = expire.get_expired_time();
                    if expired_time < Instant::now() {
                        session_id_vec.push(expire.get_session_id());

                        return false;
                    }

                    return true;
                });

                //reset check_timing
                match expires.peek() {
                    Some(expire) => {
                        let next_expired_time = expire.get_expired_time();

                        check_timing = next_expired_time - Instant::now();
                    }
                    None => {
                        check_timing = session_manager.duration;
                    }
                };

                // Check again
                if check_timing < Duration::ZERO || check_timing.is_zero() {
                    check_timing = Duration::from_millis(1);
                }

                //remove session from sessions hashmap
                let session_manager = session_manager.clone();
                tokio::spawn(async move {
                    let mut sessions = session_manager.sessions.lock().await;
                    for session_id in session_id_vec {
                        sessions.remove(&session_id);
                    }
                });
            }
        });
    }
}

#[derive(Clone)]
pub struct Session {
    pub user_id: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Expire(Instant, String);

impl Expire {
    pub fn get_expired_time(&self) -> Instant {
        self.0
    }

    pub fn get_session_id(&self) -> String {
        self.1.to_string()
    }
}

impl PartialOrd for Expire {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Expire {
    fn cmp(&self, other: &Self) -> Ordering {
        // 先比較 Duration
        match self.0.cmp(&other.0) {
            Ordering::Equal => {
                // 如果 Duration 相等，再比較 String
                self.1.cmp(&other.1)
            }
            other_ordering => other_ordering,
        }
    }
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self > other { self } else { other }
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self < other { self } else { other }
    }
}
