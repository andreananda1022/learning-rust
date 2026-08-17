use std::rc::{Rc, Weak};
use std::cell::RefCell;

// --- Bagian 1: Trait dan Implementasi Visual Palsu ---
pub trait Painter {
    fn draw(&self, element_name: &str);
}

// Implementasi default yang hanya mencetak log
struct DefaultPainter;
impl Painter for DefaultPainter {
    fn draw(&self, element_name: &str) {
        println!("Menggambar elemen: {}", element_name);
    }
}

// --- Bagian 2: Struktur Node dan Fungsinya ---
pub struct Node {
    pub name: String,
    pub painter: Box<dyn Painter>,
    pub parent: RefCell<Weak<RefCell<Node>>>,
    pub children: RefCell<Vec<Rc<RefCell<Node>>>>,
    pub is_focused: RefCell<bool>,
}

impl Node {
    pub fn new(name: &str, painter: Box<dyn Painter>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            name: String::from(name),
            painter,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
            is_focused: RefCell::new(false),
        }))
    }
    
    pub fn add_child(parent: &Rc<RefCell<Node>>, child: &Rc<RefCell<Node>>) {
        parent.borrow().children.borrow_mut().push(Rc::clone(child));
        *child.borrow().parent.borrow_mut() = Rc::downgrade(parent);
    }
    
    pub fn trigger_event(node: &Rc<RefCell<Node>>, event_name: &str) {
        let node_ref = node.borrow();
        println!("Elemen '{}' memproses event: {}", node_ref.name, event_name);
        
        // Pengecekan induk dan panggilan rekursif untuk Event Bubbling
        if let Some(parent_rc) = node_ref.parent.borrow().upgrade() {
            Self::trigger_event(&parent_rc, event_name);
        } else {
            println!("-- Perambatan event berhenti (Mencapai root / parent dihapus).");
        }
    }
}

// --- Bagian 3: Eksekusi Utama ---
fn main() {
    println!("=== 1. Membangun Hierarki UI ===");
    // Membuat elemen-elemen secara terpisah (semuanya belum punya relasi)
    let window = Node::new("Window (Root)", Box::new(DefaultPainter));
    let panel_utama = Node::new("Panel Utama", Box::new(DefaultPainter));
    let button_kirim = Node::new("Button Kirim", Box::new(DefaultPainter));

    // Merangkai hierarki: Window -> Panel -> Button
    Node::add_child(&window, &panel_utama);
    Node::add_child(&panel_utama, &button_kirim);
    
    println!("Hierarki berhasil dibangun.\n");

    println!("=== 2. Simulasi Klik dari User ===");
    // Pengguna mengklik tombol "Kirim", event dirambatkan dari bawah ke atas
    Node::trigger_event(&button_kirim, "onClick");
    
    println!("\n=== 3. Pengujian Memori (Menghancurkan Hierarki) ===");
    // Menghapus referensi ke Panel Utama secara paksa
    // Ini mensimulasikan situasi di mana elemen dihapus dari layar
    window.borrow().children.borrow_mut().clear(); // Tanpa baris ini, Panel Utama sama sekali tidak dihapus dari memori (Heap).
                                                   // Ia masih hidup karena disandera oleh vektor children di dalam window. Konsekuensinya, ketika button_kirim
                                                   // mengaktifkan referensi Weak-nya melalui .upgrade(), referensi tersebut masih berhasil menemukan wujud
                                                   // Panel Utama secara utuh. Itulah sebabnya Event Bubbling tetap merambat ke atas seolah-olah
                                                   // tidak terjadi apa-apa.
    
    drop(panel_utama);
    
    // Kita coba paksa klik tombolnya lagi setelah induknya dihancurkan
    println!("Mengklik tombol yatim piatu:");
    Node::trigger_event(&button_kirim, "onClick");
}
