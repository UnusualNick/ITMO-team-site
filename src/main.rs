use leptos::prelude::*;
use leptos::ev;
use leptos_router::components::{Router, Routes, Route, ParentRoute, Outlet};
use leptos_router::hooks::use_navigate;
use leptos_router::path;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct CommandEntry {
    cmd: String,
    out: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub tg_username: String,
    pub rank: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Achievement {
    pub id: i32,
    pub date: String,
    pub event: String,
    pub rating: i32,
    pub link: Option<String>,
}

#[component]
fn InteractiveTerminal() -> impl IntoView {
    let (history, set_history) = signal(Vec::<CommandEntry>::new());
    let (input_val, set_input) = signal(String::new());
    let (caret_pos, set_caret_pos) = signal(0_usize);
    let navigate = use_navigate();

    let execute_command = move || {
        let cmd = input_val.get();
        if cmd.trim().is_empty() { return; }
        
        let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
        let mut out = String::new();
        let mut clear_hx = false;
        
        let normalize_path = |p: &str| -> String {
            let mut dirs = Vec::new();
            for part in p.split('/') {
                if part == "." || part == "" {
                    continue;
                } else if part == ".." {
                    dirs.pop();
                } else {
                    dirs.push(part);
                }
            }
            format!("/{}", dirs.join("/"))
        };
        
        match parts[0] {
            "help" => out = "Available commands:\n  help    - Show this message\n  ls      - List directory contents\n  cd      - Change directory\n  cat     - Print file contents\n  pwd     - Print working directory\n  whoami  - Print current user\n  date    - Print current date\n  clear   - Clear terminal output\n  echo    - Display a line of text".into(),
            "pwd" => out = "/home/root".into(),
            "whoami" => out = "root".into(),
            "date" => { out = "Sat Mar 14 2026".into(); }
            "ls" => {
                let p = if parts.len() > 1 { normalize_path(parts[1]) } else { "".to_string() };
                if p == "/home" {
                    out = "welcome.sh  about.txt".into()
                } else if p == "/achievements" {
                    out = "competitions".into()
                } else if p == "/players" {
                    out = "players".into()
                } else {
                    out = "home  achievements  players".into()
                }
            },
            "cat" => {
                let p = if parts.len() > 1 { normalize_path(parts[1]) } else { "".to_string() };
                if p == "/info" || p == "/home/info" {
                    out = "[ LINKS ]\n• CTFTime:    [LINK]\n• Telegram:   [LINK]\n• Github:     [LINK]".into();
                } else if p == "/home/about.txt" || p == "/about.txt" {
                    out = "[ О КОМАНДЕ ]\nНазвание: ITMO team\nУниверситет: ИТМО\nеще что то надо бы добавить\n[ ЦЕЛИ ]\n...".into();
                } else if p == "/home/welcome.sh" || p == "/welcome.sh" {
                    out = "echo \"Welcome to ITMO team!\"".into();
                } else {
                    out = format!("cat: {}: No such file or directory", parts.get(1).unwrap_or(&""));
                }
            },
            "clear" => {
                clear_hx = true;
            },
            "echo" => {
                out = parts[1..].join(" ");
            },
            "cd" => {
                let target = if parts.len() > 1 { parts[1] } else { "/home" };
                let path = normalize_path(target);
                if path == "/home" || path == "/achievements" || path == "/players" || path == "/" {
                    navigate(&path, Default::default());
                    clear_hx = true;
                } else {
                    out = format!("bash: cd: {}: No such directory", target);
                }
            },
            _ => out = format!("bash: {}: command not found", parts[0]),
        }
        
        if clear_hx {
            set_history.set(vec![]);
        } else {
            set_history.update(|h| h.push(CommandEntry { cmd: cmd.clone(), out }));
        }
        set_input.set("".into());
        set_caret_pos.set(0);
    };

    let _ = window_event_listener(ev::keydown, move |ev: ev::KeyboardEvent| {
        if ev.ctrl_key() || ev.alt_key() || ev.meta_key() {
            if ev.key() == "c" && ev.ctrl_key() && !ev.shift_key() && !ev.alt_key() {
                set_input.set("".to_string());
                set_caret_pos.set(0);
                ev.prevent_default();
            }
            return; 
        }

        let key = ev.key();
        
        if key == "Backspace" || key == " " || key.starts_with("Arrow") || key == "Home" || key == "End" || key == "Tab" {
            ev.prevent_default();
        }

        let text = input_val.get();
        let chars: Vec<char> = text.chars().collect();
        let pos = caret_pos.get().min(chars.len());

        match key.as_str() {
            "Enter" => {
                execute_command();
            }
            "Backspace" => {
                if pos > 0 {
                    let mut new_chars = chars.clone();
                    new_chars.remove(pos - 1);
                    set_input.set(new_chars.into_iter().collect());
                    set_caret_pos.set(pos - 1);
                }
            }
            "Delete" => {
                if pos < chars.len() {
                    let mut new_chars = chars.clone();
                    new_chars.remove(pos);
                    set_input.set(new_chars.into_iter().collect());
                }
            }
            "ArrowLeft" => {
                set_caret_pos.set(pos.saturating_sub(1));
            }
            "ArrowRight" => {
                set_caret_pos.set((pos + 1).min(chars.len()));
            }
            "Home" => {
                set_caret_pos.set(0);
            }
            "End" => {
                set_caret_pos.set(chars.len());
            }
            "Tab" => {
                let current_input: String = chars.iter().collect();
                let parts: Vec<&str> = current_input.split(' ').collect();
                let last_part = parts.last().copied().unwrap_or("");
                
                if parts.len() <= 1 {
                    let commands = ["help", "pwd", "whoami", "date", "clear", "ls", "cd", "echo", "cat"];
                    let matches: Vec<&&str> = commands.iter().filter(|&&c| c.starts_with(last_part)).collect();
                    if !matches.is_empty() {
                        let new_val = format!("{} ", matches[0]);
                        set_input.set(new_val.clone());
                        set_caret_pos.set(new_val.chars().count());
                    }
                } else if parts.len() == 2 && (parts[0] == "cd" || parts[0] == "ls" || parts[0] == "cat") {
                    let (prefix, suffix) = if let Some(idx) = last_part.rfind('/') {
                        let (p, s) = last_part.split_at(idx + 1);
                        (p, s)
                    } else {
                        ("", last_part)
                    };
                    
                    let dirs = ["home", "achievements", "players", "about.txt", "info", "welcome.sh"];
                    let abs_dirs = ["/home", "/achievements", "/players", "/about.txt", "/info", "/welcome.sh"];
                    
                    let mut matched = None;
                    if prefix.is_empty() {
                        for d in abs_dirs.iter().chain(dirs.iter()) {
                            if d.starts_with(suffix) {
                                matched = Some(*d);
                                break;
                            }
                        }
                    } else {
                        for d in dirs.iter() {
                            if d.starts_with(suffix) {
                                matched = Some(*d);
                                break;
                            }
                        }
                    }
                    
                    if let Some(m) = matched {
                        let new_val = format!("{} {}{} ", parts[0], prefix, m);
                        set_input.set(new_val.clone());
                        set_caret_pos.set(new_val.chars().count());
                    }
                }
            }
            k if k.chars().count() == 1 => {
                let mut new_chars = chars.clone();
                new_chars.insert(pos, k.chars().next().unwrap());
                set_input.set(new_chars.into_iter().collect());
                set_caret_pos.set(pos + 1);
            }
            _ => {}
        }
    });

    view! {
        <div class="interactive-terminal w-full mt-4 pb-12 cursor-text">
            {move || history.get().into_iter().map(|entry| view! {
                <div class="mb-2">
                    <div class="command-line text-[0.95rem]">
                        <span class="prompt font-bold" style="color: #55aaff;">"root@itmo-team:~$"</span> " " {entry.cmd}
                    </div>
                    {
                        if entry.out.is_empty() {
                            view! { <span/> }.into_any()
                        } else {
                            view! {
                                <div class="command-output text-[#00ff00] mt-1 whitespace-pre-wrap text-[0.95rem]">
                                    {entry.out}
                                </div>
                            }.into_any()
                        }
                    }
                </div>
            }).collect_view()}
            
            <div class="relative flex items-center w-full mt-1 font-mono text-[0.95rem]">
                <style>"
                    @keyframes blink-caret { 0%, 100% { opacity: 1; } 50% { opacity: 0; } }
                    .animate-blink { animation: blink-caret 1s step-end infinite; }
                    .caret-block { background-color: #00ff00; color: #0a0e14; }
                "</style>
                <span class="prompt font-bold" style="color: #55aaff;">"root@itmo-team:~$"</span>
                
                <div class="relative ml-2 flex-grow h-6 flex items-center">
                    <div class="absolute inset-0 text-[#00ff00] pointer-events-none whitespace-pre flex items-center">
                        {move || {
                            let text = input_val.get();
                            let chars: Vec<char> = text.chars().collect();
                            let pos = caret_pos.get().min(chars.len());
                            
                            let before: String = chars[..pos].iter().collect();
                            let active_char = if pos < chars.len() { chars[pos].to_string() } else { " ".to_string() };
                            let after: String = if pos + 1 < chars.len() { chars[pos+1..].iter().collect() } else { "".to_string() };
                            
                            view! {
                                <span>{before}</span>
                                <span class="animate-blink caret-block">{active_char}</span>
                                <span>{after}</span>
                            }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { "Not Found" }>
                <ParentRoute path=path!("") view=Layout>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/home") view=Home />
                    <Route path=path!("/achievements") view=Achievements />
                    <Route path=path!("/players") view=Players />
                </ParentRoute>
                <Route path=path!("*any") view=Feedback />
            </Routes>
        </Router>
    }
}

#[component]
fn Layout() -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let is_active = move |path: &'static str| {
        let p = location.pathname.get();
        if path == "/home" && (p == "/" || p == "/home") {
            true
        } else {
            p.starts_with(path)
        }
    };

    let style_for = move |path: &'static str| {
        move || {
            if is_active(path) {
                "color: #55aaff; text-decoration: none;"
            } else {
                "color: #00ff00; text-decoration: none;"
            }
        }
    };

    view! {
        <div class="terminal font-mono min-h-screen flex flex-col" style="background-color: #0a0e14; color: #00ff00;">
            <header class="flex justify-between items-center py-4 px-5 border-b sticky top-0 z-50 flex-shrink-0" style="background-color: rgba(10, 14, 20, 0.98); border-color: rgba(0, 255, 0, 0.2);">
                <div class="text-[1.8rem] font-bold tracking-normal" style="color: #00ff00;">"ITMO_team"</div>
                <nav class="flex gap-8 text-[0.95rem]">
                    <a href="/home" class="transition-colors hover:opacity-80" style=style_for("/home")>"/home"</a>
                    <a href="/achievements" class="transition-colors hover:opacity-80" style=style_for("/achievements")>"/achievements"</a>
                    <a href="/players" class="transition-colors hover:opacity-80" style=style_for("/players")>"/players"</a>
                </nav>
            </header>

            <div id="content" class="content flex-grow flex flex-col px-5 py-6">
                <div class="terminal-output flex-grow">
                    <Outlet />

                    <div class="command-line mt-[15px] mb-[15px] text-[0.95rem]">
                        <span class="prompt font-bold" style="color: #55aaff;">"root@itmo-team:~$"</span> " cat ./info"
                    </div>
                    <div class="command-output mt-[10px] mb-[20px] px-0 py-[10px]">
                        <pre class="whitespace-pre-wrap text-[0.95rem] leading-relaxed font-mono" style="margin: 0;">
"[ LINKS ]\n"
"• CTFTime:    "<a style="color: #ffaa00; text-decoration: none;" href="https://ctftime.org/team/283875/" target="_blank">"[LINK]"</a>"\n"
"• Telegram:   "<a style="color: #ffaa00; text-decoration: none;" href="https://t.me/it_sspace" target="_blank">"[LINK]"</a>"\n"
"• Github:     "<a style="color: #ffaa00; text-decoration: none;" href="https://www.youtube.com/watch?v=dQw4w9WgXcQ" target="_blank">"[LINK]"</a>
                        </pre>
                    </div>

                    <InteractiveTerminal />
                </div>
            </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <div class="command-line mt-[15px] mb-[15px]">
                <span class="prompt font-bold" style="color: #55aaff;">"root@itmo-team:~$"</span> " ./welcome.sh"
            </div>
            <div class="command-output mt-[10px] mb-[20px] px-0 py-[10px]">
                <pre class="ascii-art whitespace-pre overflow-x-auto" style="font-size: 0.9rem; line-height: 1.2; margin: 10px 0;">
"╔═══════════════════════════════════════════════════╗
║                                                   ║
║    ██╗████████╗███╗   ███╗ ██████╗                ║
║    ██║╚══██╔══╝████╗ ████║██╔═══██╗               ║
║    ██║   ██║   ██╔████╔██║██║   ██║               ║
║    ██║   ██║   ██║╚██╔╝██║██║   ██║               ║
║    ██║   ██║   ██║ ╚═╝ ██║╚██████╔╝               ║
║    ╚═╝   ╚═╝   ╚═╝     ╚═╝ ╚═════╝                ║
║                                                   ║
║         ████████╗███████╗ █████╗ ███╗   ███╗      ║
║         ╚══██╔══╝██╔════╝██╔══██╗████╗ ████║      ║
║            ██║   █████╗  ███████║██╔████╔██║      ║
║            ██║   ██╔══╝  ██╔══██║██║╚██╔╝██║      ║
║            ██║   ███████╗██║  ██║██║ ╚═╝ ██║      ║
║            ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝      ║
║                                                   ║
╚═══════════════════════════════════════════════════╝"
                </pre>
            </div>
            <div class="command-line mt-[15px] mb-[15px]">
                <span class="prompt font-bold" style="color: #55aaff;">"root@itmo-team:~$"</span> " cat about.txt"
            </div>
            <div class="command-output mt-[10px] mb-[20px] px-0 py-[10px]">
                <pre class="whitespace-pre-wrap text-[0.95rem] leading-relaxed font-mono" style="margin: 0;">
"[ О КОМАНДЕ ]
Название: ITMO team
Университет: ИТМО
еще что то надо бы добавить
[ ЦЕЛИ ]
..."
                </pre>
            </div>
            <div class="command-line mt-[15px] mb-[15px]">
                <span class="prompt font-bold" style="color: #55aaff;">"root@itmo-team:~$"</span> " help"
            </div>
            <div class="command-output mt-[10px] mb-[20px] px-0 py-[10px]">
                <pre class="whitespace-pre-wrap text-[0.95rem] leading-relaxed font-mono" style="margin: 0;">
"Available commands:
  help    - Show this message
  ls      - List directory contents
  cd      - Change directory
  cat     - Print file contents
  pwd     - Print working directory
  whoami  - Print current user
  date    - Print current date
  clear   - Clear terminal output
  echo    - Display a line of text"
                </pre>
            </div>
        </div>
    }
}

async fn fetch_achievements() -> Vec<Achievement> {
    let res = reqwest::get("http://127.0.0.1:3000/api/achievements").await;
    match res {
        Ok(r) => {
            if let Ok(jsx) = r.json::<Vec<Achievement>>().await {
                jsx
            } else {
                vec![]
            }
        }
        Err(_) => vec![]
    }
}

async fn fetch_players() -> Vec<Player> {
    let res = reqwest::get("http://127.0.0.1:3000/api/players").await;
    match res {
        Ok(r) => {
            if let Ok(jsx) = r.json::<Vec<Player>>().await {
                jsx
            } else {
                vec![]
            }
        }
        Err(_) => vec![]
    }
}

#[component]
fn Achievements() -> impl IntoView {
    let raw_achievements = LocalResource::new(|| async move { fetch_achievements().await });

    view! {
        <div>
            <div class="command-line mt-[15px] mb-[15px]">
                <span class="prompt font-bold" style="color: #55aaff;">"root@itmo-team:~$"</span> " ls -la ./competitions/"
            </div>
            <div class="command-output mt-[10px] mb-[20px] px-0 py-[10px]">
                <div class="competitions-table-wrapper w-full mb-[15px]">
                    <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                        {move || {
                            raw_achievements.get().map(|data| {
                                let view_rows = data.into_iter().map(|acc| {
                                    view! {
                                        <tr>
                                            <td class="font-normal p-2 border-b border-green-500/20" style="color: #ff5555; width: 100px;">{acc.date}</td>
                                            <td class="font-normal p-2 border-b border-green-500/20">{acc.event}</td>
                                            <td class="font-normal p-2 border-b border-green-500/20 text-center" style="color: #ffaa00; width: 50px;">{acc.rating}</td>
                                            <td class="font-normal p-2 border-b border-green-500/20 text-center" style="width: 70px;">
                                                {if let Some(l) = acc.link {
                                                    view! { <a href=l target="_blank" style="color: #ffaa00; text-decoration: none;">"[LINK]"</a> }.into_any()
                                                } else {
                                                    view! { <span>"-"</span> }.into_any()
                                                }}
                                            </td>
                                        </tr>
                                    }
                                }).collect_view();
                                
                                view! {
                                    <table class="competitions-table w-full text-left border-collapse text-[0.9rem] font-mono">
                                        <thead>
                                            <tr>
                                                <th class="font-normal p-2 border-b border-green-500/50" style="color: #00ff00; width: 100px;">"Дата"</th>
                                                <th class="font-normal p-2 border-b border-green-500/50 min-w-[250px]" style="color: #00ff00;">"Соревнование"</th>
                                                <th class="font-normal p-2 border-b border-green-500/50 text-center" style="color: #00ff00; width: 50px;">"Место"</th>
                                                <th class="font-normal p-2 border-b border-green-500/50 text-center" style="color: #00ff00; width: 70px;">"Ссылка"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {view_rows}
                                        </tbody>
                                    </table>
                                }
                            })
                        }}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Players() -> impl IntoView {
    let raw_players = LocalResource::new(|| async move { fetch_players().await });

    view! {
        <div>
            <div class="command-line mt-[15px] mb-[15px]">
                <span class="prompt font-bold" style="color: #55aaff;">"root@itmo-team:~$"</span> " ls -la ./players/"
            </div>
            <div class="command-output mt-[10px] mb-[20px] px-0 py-[10px]">
                <div class="competitions-table-wrapper w-full mb-[15px]">
                    <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                        {move || {
                            raw_players.get().map(|data| {
                                let view_rows = data.into_iter().map(|p| {
                                    view! {
                                        <tr>
                                            <td class="font-normal p-2 border-b border-green-500/20">{p.name}</td>
                                            <td class="font-normal p-2 border-b border-green-500/20" style="color: #ff5555;">{p.rank}</td>
                                            <td class="font-normal p-2 border-b border-green-500/20 text-center" style="color: #ffaa00; width: 70px;">
                                                <a href=format!("https://t.me/{}", p.tg_username.clone().trim_start_matches('@')) target="_blank" style="color: #ffaa00; text-decoration: none;">{p.tg_username.clone()}</a>
                                            </td>
                                        </tr>
                                    }
                                }).collect_view();

                                view! {
                                    <table class="competitions-table w-full text-left border-collapse text-[0.9rem] font-mono">
                                        <thead>
                                            <tr>
                                                <th class="font-normal p-2 border-b border-green-500/50 min-w-[250px]" style="color: #00ff00;">"Игрок"</th>
                                                <th class="font-normal p-2 border-b border-green-500/50 min-w-[250px]" style="color: #00ff00;">"Звание"</th>
                                                <th class="font-normal p-2 border-b border-green-500/50 text-center" style="color: #00ff00; width: 70px;">"Контакт"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {view_rows}
                                        </tbody>
                                    </table>
                                }
                            })
                        }}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Feedback() -> impl IntoView {
    view! { "feedback" }
}

fn main() { leptos::mount::mount_to_body(App) }
