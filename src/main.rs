#![windows_subsystem = "windows"]

use std::collections::HashSet;
use std::ffi::c_void;
use std::mem::zeroed;
use std::ptr::{null, null_mut};

use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::SystemInformation::{GetLocalTime, GetTickCount};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
use windows_sys::Win32::UI::WindowsAndMessaging::*;

const BG: COLORREF = 0x001b1818;
const BG2: COLORREF = 0x0023231f;
const INPUT_BG: COLORREF = 0x00352f2f;
const TEXT: COLORREF = 0x00f1efef;
const MUTED: COLORREF = 0x00b8adad;
const PURPLE: COLORREF = 0x00ff4791;
const RED: COLORREF = 0x001619e9;

const NICK_COLORS: [COLORREF; 12] = [
    0x000045ff, 0x00ff901e, 0x007fff00, 0x00b469ff, 0x0032cd9a, 0x0020a5da, 0x00e22b8a,
    0x00d1ce00, 0x007fd3ff, 0x001e69d2, 0x00a09e5f, 0x000000ff,
];

const NAMES: &[&str] = &[
    "eaglepham", "cheesoonator22", "Meva400", "Nightbot", "fancy_pantz", "mimihrafnkels",
    "SakelJo", "aero997", "lyandra", "maddogos", "Keibug001", "MultipleMean150", "Flippy978",
    "kekw_master", "tolya_kek", "xXx_snayper", "mama_ya_v_tanke", "sanya228", "kotleta_2007",
    "vitalya_tt", "zhenya_pro", "danya_krut", "masha_na_chate", "stariy_ded", "tuman_pro",
    "artem_gg", "kirill_ff", "nastya_kotik", "roma_boom", "lesha_penza", "vova_kek",
    "serega_tyt", "nikita_sup", "yulia_moon", "anton_awp", "dima_dno", "ilya_top", "max_sniper",
    "oleg_pivo", "pasha_boss", "rita_tea", "sasha_gg", "timur_king", "ulyana_star", "yana_fox",
    "zakhar_pro", "grisha_cheburek", "petya_lag", "kostya_gg", "lena_chan",
];

const AFK: &[&str] = &[
    "стример в АФК", "завис", "ало, стример?", "он умер?", "афк уже {n} минут",
    "вернись, я всё прощу", "чат, он живой?", "походу уснул", "стример вышел покурить",
    "микрофон дышит, а его нет", "стрим умер, чат живёт", "опять афк", "ну и где он",
    "он там чай пьёт?", "стример, ты тут?", "вышел за хлебом",
    "я пока поем, он всё равно спит", "каждый стрим одно и то же",
    "пока он афк, расскажите как у вас дела", "кто-нибудь знает где он",
    "уже {n} минут тишины", "он в туалете застрял", "вернулся? нет",
    "стример, моргни два раза", "чат, ставьте таймер", "он там вообще есть?",
    "я один тут сижу?", "стример ушёл, чат остался", "завис намертво",
    "перезагрузите стримера", "афк, как всегда", "он ушёл в астрал",
    "стрим пошёл по пизде", "я заплатил за это?", "ну и ладно, чат развлекает",
    "стример спит, я смотрю", "ало, гараж", "слышно только кулер",
    "он там доширак делает", "уже час афк, я привык", "стример, ты где",
    "чат, го флудить", "афк-стрим, классика", "он просто медитирует", "ну сколько можно",
];

const CS2: &[&str] = &[
    "опять кс", "го раш б", "почему не берёшь AWP", "этот раунд был изи", "эко раунд, чат",
    "нуб", "он опять купил дигл", "аим как у бота", "кт сливают", "снайпер, ты где",
    "го некст", "ммр растёт", "чё за раскидка", "ну это фейс", "смоук на мид",
    "флешку кидай", "ну всё, 16:14", "клатч?", "лагает или он такой",
    "смотри на миникарту", "почему не смотришь за спину", "это было чисто",
    "ну и зачем ты туда пошёл", "герой раунда", "я в него верил", "го на длинную",
    "эко, потом фулл", "опять тимейты дно", "он в кс2 опять", "почему у тебя 40 фпс",
    "смотри, у него 30 фрагов", "ну это же изи фраг",
];

const BEAMNG: &[&str] = &[
    "краш тест пошёл", "опять разбил машину", "физика огонь", "beamng drive топ",
    "эту тачку в хлам", "го на трассу", "движок как настоящий", "колесо отлетело",
    "смотри как сложилась", "симулятор автокатастрофы", "го в гору", "на 300 км/ч",
    "подвеска умерла", "ну это боль", "красиво вошёл в столб", "опять beamng",
    "тачку в тотал", "физика сломалась", "го в дрифт", "этот краш на 10 из 10",
    "он разбил ещё одну", "рама погнулась", "го с трамплина",
    "колёса отдельно, кузов отдельно", "красиво летит", "ну это уже не машина",
    "beamng это симулятор ремонта", "смотри как кузов сложился",
];

const YT: &[&str] = &[
    "что смотришь", "видос топ", "го дальше", "это же было в рекомендациях",
    "обложка у {title} кликбейт", "{title} — топчик", "а можно ссылку",
    "пересматриваю вместе с тобой", "ютуб смотрим, норм", "смотри до конца",
    "опять ютуб", "реклама скип", "этот ролик видел", "чату тоже интересно",
    "стример смотрит ютуб, чат работает", "го на 2х скорость", "на чём остановился",
    "а дальше что было", "смотрел {title} вчера", "обложка у этого видео ор",
    "название видео кликбейт", "го плейлист", "сколько можно ютуб смотреть",
    "я тоже так хочу", "этот видос легенда", "го пересматривать",
];

const VIBE: &[&str] = &[
    "вайбкодим потихоньку", "промпт и в прод", "нейронка, сделай красиво",
    "он даже не читает что генерит", "опять скопировал из чата ИИ", "вайбкод не баг, а фича",
    "тесты? зачем, оно же вайбит", "он щас промпт дописывает", "нейронка опять всё сломала",
    "спросил у ИИ, ИИ сказал работает", "деплой в пятницу, вайб", "оно работает, не трогай",
    "мне ИИ так написал", "вайбкодинг это когда не читаешь", "чат, он реально вайбкодит",
    "ещё один промпт и всё", "он просит ИИ починить то, что ИИ сломал",
    "нейронка сказала что всё ок", "я тоже так кодил", "зачем думать, есть же ИИ",
    "промпт-инженер года", "он даже ошибку не читает", "вайб зашкаливает",
    "изи, нейронка всё сделает", "это же вайбкод, так и надо",
    "он там с ИИ разговаривает", "пять промптов и готово", "оно само собралось",
    "не баг, а вайб", "чат, смотрите как он вайбкодит",
    "ИИ написал, он запустил, всё работает", "вайбкодинг до первого бага",
    "нейронка уже третий час чинит", "он верит в ИИ больше чем в себя",
    "промпт длиннее кода", "зачем ревью, если есть вайб",
];

const DESKTOP: &[&str] = &[
    "привет чат", "стрим идёт", "что происходит", "первый", "KEKW", "LUL", "OMEGALUL",
    "классный стрим", "смотрю с телефона", "ру твич лучший", "чат, вы тут?",
    "стример, го что-нибудь", "а что сегодня по плану", "ну наконец-то",
    "KEKW KEKW KEKW", "ну это классика", "я тут новенький", "подписался, норм",
    "го общаться", "стрим как всегда топ", "ало, чат", "скиньте смайл", "PogChamp",
    "PepeHands", "ну и тишина",
];

const LH: i32 = 17;

#[derive(Clone, PartialEq)]
enum Ctx {
    Afk,
    Game(String),
    Youtube(String),
    Code,
    Desktop,
}

struct Msg {
    time: String,
    name: String,
    color: COLORREF,
    text: String,
}

struct App {
    msgs: Vec<Msg>,
    timer: f64,
    ctx: Ctx,
    idle: u64,
    used: HashSet<String>,
    seed: u32,
    title: String,
    corpus: Vec<&'static str>,
}

impl App {
    fn rnd(&mut self) -> u32 {
        let mut x = self.seed;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.seed = x;
        x
    }

    fn pick<'a>(&mut self, pool: &[&'a str]) -> &'a str {
        pool[(self.rnd() as usize) % pool.len()]
    }

    fn range(&mut self, a: i32, b: i32) -> i32 {
        a + (self.rnd() % ((b - a) as u32)) as i32
    }

    fn chance(&mut self, pct: u32) -> bool {
        self.rnd() % 100 < pct
    }

    fn interval(&mut self) -> f64 {
        0.95 + (self.rnd() % 50) as f64 / 100.0
    }

    fn make_text(&mut self) -> String {
        if !self.corpus.is_empty() && self.chance(45) {
            let i = (self.rnd() as usize) % self.corpus.len();
            return self.corpus[i].to_string();
        }
        let (pool, title): (&[&str], Option<String>) = match &self.ctx {
            Ctx::Afk => (AFK, None),
            Ctx::Game(g) if g == "CS2" => (CS2, None),
            Ctx::Game(_) => (BEAMNG, None),
            Ctx::Youtube(t) => (YT, Some(t.clone())),
            Ctx::Code => (VIBE, None),
            Ctx::Desktop => (DESKTOP, None),
        };
        let mut s = self.pick(pool).to_string();
        if s.contains("{n}") {
            let n = self.range(2, 45);
            s = s.replace("{n}", &n.to_string());
        }
        if s.contains("{title}") {
            let t = title.unwrap_or_default();
            let short: String = if t.is_empty() { "видос".into() } else { t.chars().take(38).collect() };
            s = s.replace("{title}", &short);
        }
        if self.chance(18) {
            s.push_str(" KEKW");
        } else if self.chance(8) {
            s.push_str(" LUL");
        }
        s
    }

    fn push_msg(&mut self, name: &str, text: &str) {
        let color = NICK_COLORS[name.bytes().map(|b| b as usize).sum::<usize>() % NICK_COLORS.len()];
        self.msgs.push(Msg { time: now_hhmm(), name: name.to_string(), color, text: text.to_string() });
        if self.msgs.len() > 200 {
            self.msgs.remove(0);
        }
    }

    fn generate(&mut self) {
        let mut text = self.make_text();
        if self.used.contains(&text) {
            for _ in 0..12 {
                text = self.make_text();
                if !self.used.contains(&text) {
                    break;
                }
            }
        }
        if self.used.contains(&text) {
            let tail = self.pick(&[" KEKW", " LUL", " xD", " OMEGALUL", " ахах", " PogChamp"]);
            text.push_str(tail);
        }
        self.used.insert(text.clone());
        if self.used.len() > 4000 {
            self.used.clear();
        }
        let name = self.pick(NAMES).to_string();
        self.push_msg(&name, &text);
    }

    fn tick(&mut self, dt: f64) {
        self.idle = idle_secs();
        let new_ctx = if self.idle >= 60 { Ctx::Afk } else { detect(&active_title()) };
        if new_ctx != self.ctx {
            let old = std::mem::replace(&mut self.ctx, new_ctx);
            if matches!(old, Ctx::Afk) {
                self.push_msg("Nightbot", "стример вернулся, погнали");
            }
            self.timer = 0.0;
        }
        self.timer -= dt;
        let mut guard = 0;
        while self.timer <= 0.0 && guard < 60 {
            self.generate();
            self.timer += self.interval();
            guard += 1;
        }
    }

    fn label(&self) -> String {
        match &self.ctx {
            Ctx::Afk => format!("Twitch Chat — AFK {}s", self.idle),
            Ctx::Game(g) => format!("Twitch Chat — {g}"),
            Ctx::Youtube(t) => format!("Twitch Chat — YouTube: {t}"),
            Ctx::Code => "Twitch Chat — opencode".to_string(),
            Ctx::Desktop => "Twitch Chat".to_string(),
        }
    }
}

static mut APP: *mut App = null_mut();

fn now_hhmm() -> String {
    unsafe {
        let mut st: SYSTEMTIME = zeroed();
        GetLocalTime(&mut st);
        format!("{:02}:{:02}", st.wHour, st.wMinute)
    }
}

fn idle_secs() -> u64 {
    unsafe {
        let mut li = LASTINPUTINFO { cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32, dwTime: 0 };
        GetLastInputInfo(&mut li);
        (GetTickCount().wrapping_sub(li.dwTime) / 1000) as u64
    }
}

fn active_title() -> String {
    unsafe {
        let mut buf = [0u16; 512];
        let n = GetWindowTextW(GetForegroundWindow(), buf.as_mut_ptr(), 512);
        String::from_utf16_lossy(&buf[..n.max(0) as usize])
    }
}

fn detect(title: &str) -> Ctx {
    let t = title.to_lowercase();
    if let Some(i) = t.find("youtube") {
        let raw = if title.is_char_boundary(i) { &title[..i] } else { &t[..i] };
        let name = raw
            .trim_end_matches(|c: char| c == '-' || c == '—' || c == '|' || c == ':' || c.is_whitespace())
            .trim()
            .to_string();
        Ctx::Youtube(name)
    } else if t.contains("counter-strike") || t.contains("cs2") || t.contains("cs 1.6") {
        Ctx::Game("CS2".into())
    } else if t.contains("beamng") {
        Ctx::Game("BeamNG.drive".into())
    } else if t.contains("opencode") || t.contains("visual studio code") {
        Ctx::Code
    } else {
        Ctx::Desktop
    }
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

unsafe fn font(size: i32, weight: i32) -> HFONT {
    CreateFontW(
        -size, 0, 0, 0, weight, 0, 0, 0, DEFAULT_CHARSET as u32, 0, 0,
        CLEARTYPE_QUALITY as u32, 0, wide("Segoe UI").as_ptr(),
    )
}

unsafe fn text(hdc: HDC, x: i32, y: i32, s: &str, color: COLORREF, f: HFONT) {
    if s.is_empty() {
        return;
    }
    SelectObject(hdc, f);
    SetTextColor(hdc, color);
    SetBkMode(hdc, TRANSPARENT as i32);
    let w = wide(s);
    TextOutW(hdc, x, y, w.as_ptr(), (w.len() - 1) as i32);
}

unsafe fn text_w(hdc: HDC, s: &str, f: HFONT) -> i32 {
    SelectObject(hdc, f);
    let w = wide(s);
    let mut sz = SIZE { cx: 0, cy: 0 };
    GetTextExtentPoint32W(hdc, w.as_ptr(), (w.len() - 1) as i32, &mut sz);
    sz.cx
}

unsafe fn wrap(hdc: HDC, s: &str, maxw: i32, f: HFONT) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for word in s.split(' ') {
        let cand = if cur.is_empty() { word.to_string() } else { format!("{cur} {word}") };
        if !cur.is_empty() && text_w(hdc, &cand, f) > maxw {
            out.push(std::mem::take(&mut cur));
            cur = word.to_string();
        } else {
            cur = cand;
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

unsafe fn fill(hdc: HDC, l: i32, t: i32, r: i32, b: i32, color: COLORREF) {
    let brush = CreateSolidBrush(color);
    let rect = RECT { left: l, top: t, right: r, bottom: b };
    FillRect(hdc, &rect, brush);
    DeleteObject(brush);
}

unsafe fn paint(hwnd: HWND) {
    let mut ps: PAINTSTRUCT = zeroed();
    let hdc = BeginPaint(hwnd, &mut ps);
    let mut rc: RECT = zeroed();
    GetClientRect(hwnd, &mut rc);
    let (w, h) = (rc.right, rc.bottom);
    let app = &mut *APP;

    let mem = CreateCompatibleDC(hdc);
    let bmp = CreateCompatibleBitmap(hdc, w, h);
    let old = SelectObject(mem, bmp);

    fill(mem, 0, 0, w, h, BG);

    let f_small = font(11, FW_NORMAL as i32);
    let f_msg = font(13, FW_NORMAL as i32);
    let f_bold = font(13, FW_BOLD as i32);

    text(mem, 10, 14, "←|", TEXT, f_bold);
    let hdr = "STREAM CHAT";
    text(mem, (w - text_w(mem, hdr, f_bold)) / 2, 14, hdr, TEXT, f_bold);
    text(mem, w - 24, 14, "1", TEXT, f_small);

    let ch_top = 40;
    fill(mem, 0, ch_top, w, ch_top + 1, 0x00302b2b);
    let brush = CreateSolidBrush(0x002b5a8a);
    SelectObject(mem, brush);
    Ellipse(mem, 12, ch_top + 14, 46, ch_top + 48);
    DeleteObject(brush);
    text(mem, 56, ch_top + 16, "eaglepham", TEXT, f_bold);
    text(mem, 56, ch_top + 33, "★ 25,000", RED, f_small);
    fill(mem, 0, ch_top + 62, w, ch_top + 63, 0x00302b2b);

    let in_top = h - 46;
    fill(mem, 0, in_top, w, h, BG);
    fill(mem, 10, in_top + 7, w - 10, in_top + 37, INPUT_BG);
    text(mem, 20, in_top + 15, "Send a message", MUTED, f_msg);

    let chat_top = ch_top + 63;
    let chat_bottom = in_top;
    fill(mem, 0, chat_top, w, chat_bottom, BG2);
    let mut y = chat_bottom;
    for m in app.msgs.iter().rev() {
        let tpre = format!("{} ", m.time);
        let npre = format!("{}: ", m.name);
        let pw = text_w(mem, &tpre, f_small) + text_w(mem, &npre, f_msg);
        let lines = wrap(mem, &m.text, w - 16 - pw, f_msg);
        if lines.is_empty() {
            continue;
        }
        let height = (lines.len() as i32) * LH + 5;
        y -= height;
        if y < chat_top {
            break;
        }
        let mut ly = y + 2;
        text(mem, 8, ly + 1, &tpre, MUTED, f_small);
        text(mem, 8 + text_w(mem, &tpre, f_small), ly, &npre, m.color, f_msg);
        text(mem, 8 + pw, ly, &lines[0], TEXT, f_msg);
        ly += LH;
        for l in &lines[1..] {
            text(mem, 8, ly, l, TEXT, f_msg);
            ly += LH;
        }
    }

    BitBlt(hdc, 0, 0, w, h, mem, 0, 0, SRCCOPY);
    SelectObject(mem, old);
    DeleteObject(bmp);
    DeleteDC(mem);
    DeleteObject(f_small);
    DeleteObject(f_msg);
    DeleteObject(f_bold);
    EndPaint(hwnd, &ps);
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            let mut app = Box::new(App {
                msgs: Vec::new(),
                timer: 0.0,
                ctx: Ctx::Desktop,
                idle: 0,
                used: HashSet::new(),
                seed: 0x9e3779b9 ^ GetTickCount(),
                title: String::new(),
                corpus: include_str!("../assets/phrases.txt")
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .collect(),
            });
            for _ in 0..6 {
                app.generate();
            }
            APP = Box::into_raw(app);
            SetTimer(hwnd, 1, 200, None);
            SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
            0
        }
        WM_TIMER => {
            if !APP.is_null() {
                (*APP).tick(0.2);
                let label = (*APP).label();
                if label != (*APP).title {
                    (*APP).title = label.clone();
                    SetWindowTextW(hwnd, wide(&label).as_ptr());
                }
                InvalidateRect(hwnd, null(), 0);
            }
            0
        }
        WM_PAINT => {
            paint(hwnd);
            0
        }
        WM_ERASEBKGND => 1,
        WM_DESTROY => {
            if !APP.is_null() {
                drop(Box::from_raw(APP));
                APP = null_mut();
            }
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wp, lp),
    }
}

fn main() {
    unsafe {
        SetProcessDPIAware();
        let hinst = GetModuleHandleW(null());
        let class = wide("FakeTwitchChat");
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinst,
            hIcon: null_mut(),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: null_mut(),
            lpszMenuName: null(),
            lpszClassName: class.as_ptr(),
        };
        RegisterClassW(&wc);
        let hwnd = CreateWindowExW(
            0,
            class.as_ptr(),
            wide("Twitch Chat").as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            80,
            60,
            396,
            760,
            null_mut(),
            null_mut(),
            hinst,
            null(),
        );
        if hwnd.is_null() {
            return;
        }
        let mut msg: MSG = zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
