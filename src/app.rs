use crate::models::Quiz;
use crate::quiz_loader::{load_quiz_list, load_quizzes};
use eframe::egui;

pub struct MyApp {
    questions: Vec<Quiz>,
    current_question_index: usize,
    selected_answer: Option<usize>,
    user_answers: Vec<Option<String>>,
    show_results: bool,
    quiz_path: String,
    quiz_files: Vec<String>,
    show_quiz_selection: bool,
    quizzes_dir: String,
    image_cache: std::collections::HashMap<String, Vec<u8>>, // cache des bytes des images
}

impl MyApp {
    pub fn new(quiz_path: &str) -> Self {
        // Si quiz_path est un dossier, l'utiliser directement, sinon extraire le parent
        let quizzes_dir = if std::path::Path::new(quiz_path).is_dir() {
            quiz_path.to_string()
        } else {
            std::path::Path::new(quiz_path)
                .parent()
                .and_then(|p| p.to_str())
                .unwrap_or("Quizzes")
                .to_string()
        };
        
        let quiz_files = load_quiz_list(&quizzes_dir);
        let show_quiz_selection = quiz_files.len() > 1;
        
        // Si un seul quiz, le charger automatiquement
        let questions = if quiz_files.len() == 1 {
            let first_quiz = format!("{}/{}", quizzes_dir, quiz_files[0]);
            load_quizzes(&first_quiz)
        } else if show_quiz_selection {
            Vec::new()
        } else {
            load_quizzes(quiz_path)
        };
        
        let user_answers = vec![None; questions.len()];
        Self {
            questions,
            current_question_index: 0,
            selected_answer: None,
            user_answers,
            show_results: false,
            quiz_path: quiz_path.to_string(),
            quiz_files,
            show_quiz_selection,
            quizzes_dir,
            image_cache: std::collections::HashMap::new(),
        }
    }
    
    fn load_selected_quiz(&mut self, filename: &str) {
        let path = format!("{}/{}", self.quizzes_dir, filename);
        self.questions = load_quizzes(&path);
        self.user_answers = vec![None; self.questions.len()];
        self.current_question_index = 0;
        self.selected_answer = None;
        self.show_results = false;
        self.show_quiz_selection = false;
        self.quiz_path = path;
        self.image_cache.clear(); // réinitialiser le cache pour le nouveau quiz
    }

    fn show_quiz_screen(&mut self, ui: &mut egui::Ui) {
        // Header avec titre et numéro de question
        egui::TopBottomPanel::top("quiz_header").show_inside(ui, |ui| {
            ui.add_space(10.0);
            ui.heading("Quiz");
            ui.label(format!("Question {}/{}", self.current_question_index + 1, self.questions.len()));
            ui.add_space(10.0);
        });
        
        egui::CentralPanel::default().show_inside(ui, |ui| {
            if let Some(current_quiz) = self.questions.get(self.current_question_index) {
                // Zone pour la question (ne prend que l'espace nécessaire)
                ui.vertical(|ui| {
                    ui.label(&current_quiz.question);
                    ui.add_space(15.0);
                });
                
                // Restaurer la réponse précédemment sélectionnée
                if self.selected_answer.is_none() {
                    if let Some(Some(saved_letter)) = self.user_answers.get(self.current_question_index) {
                        for (i, answer) in current_quiz.answers.iter().enumerate() {
                            if &answer.letter == saved_letter {
                                self.selected_answer = Some(i);
                                break;
                            }
                        }
                    }
                }
                
                // Calculer la taille disponible et la diviser
                let available_width = ui.available_width();
                let available_height = ui.available_height();
                let spacing = 10.0;
                let cell_width = (available_width - spacing) / 2.0;
                
                // Calculer le nombre de lignes nécessaires
                let num_answers = current_quiz.answers.len();
                let num_rows = (num_answers + 1) / 2;
                let cell_height = if num_rows > 0 {
                    (available_height - (spacing * (num_rows - 1) as f32)) / num_rows as f32
                } else {
                    available_height
                };
                
                // Grille avec cellules de taille fixe
                egui::Grid::new("answers_grid")
                    .num_columns(2)
                    .spacing([spacing, spacing])
                    .show(ui, |ui| {
                        for (i, answer) in current_quiz.answers.iter().enumerate() {
                            let is_selected = self.selected_answer == Some(i);
                            
                            // Frame cliquable pour chaque réponse avec taille fixe
                            let (rect, frame_resp) = ui.allocate_exact_size(
                                egui::vec2(cell_width, cell_height),
                                egui::Sense::click()
                            );
                            
                            if frame_resp.clicked() {
                                self.selected_answer = Some(i);
                                self.user_answers[self.current_question_index] = Some(answer.letter.clone());
                            }
                            
                            // Dessiner le cadre de la cellule
                            let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(rect));
                            
                            egui::Frame::new()
                                .fill(if is_selected { 
                                    child_ui.style().visuals.selection.bg_fill 
                                } else { 
                                    child_ui.style().visuals.widgets.inactive.bg_fill 
                                })
                                .stroke(if is_selected {
                                    egui::Stroke::new(2.0, child_ui.style().visuals.selection.stroke.color)
                                } else {
                                    egui::Stroke::new(1.0, child_ui.style().visuals.widgets.inactive.bg_stroke.color)
                                })
                                .corner_radius(5.0)
                                .inner_margin(10.0)
                                .show(&mut child_ui, |ui| {
                                    ui.vertical_centered(|ui| {
                                        // Image si disponible
                                        if let Some(image_path) = &answer.image {
                                            // Charger depuis le cache ou lire depuis le disque
                                            let bytes_opt = if let Some(cached) = self.image_cache.get(image_path) {
                                                Some(cached.clone())
                                            } else {
                                                match std::fs::read(image_path) {
                                                    Ok(bytes) => {
                                                        self.image_cache.insert(image_path.clone(), bytes.clone());
                                                        Some(bytes)
                                                    }
                                                    Err(_) => None,
                                                }
                                            };
                                            if let Some(bytes) = bytes_opt {
                                                // Calculer la taille max pour l'image (en laissant de l'espace pour le texte)
                                                let max_image_size = egui::vec2(
                                                    cell_width - 40.0, 
                                                    cell_height - 80.0
                                                );
                                                ui.add(
                                                    egui::Image::from_bytes(format!("bytes://{}", image_path), bytes)
                                                        .max_size(max_image_size)
                                                        .maintain_aspect_ratio(true)
                                                        .shrink_to_fit()
                                                        .corner_radius(5.0)
                                                        .show_loading_spinner(true)
                                                        .alt_text(answer.text.clone())
                                                );
                                            } else {
                                                ui.label("(image manquante)");
                                            }
                                        } else {
                                            ui.label(
                                                egui::RichText::new(format!("{}) {}", answer.letter, answer.text))
                                                    .color(if is_selected {
                                                        ui.style().visuals.strong_text_color()
                                                    } else {
                                                        ui.style().visuals.text_color()
                                                    })
                                            );
                                        }
                                    });
                                });
                            
                            if i % 2 == 1 { ui.end_row(); }
                        }
                    });
            }
        });
        
        // Footer avec les boutons de navigation (seulement si plusieurs questions ou quiz)
        if self.questions.len() > 1 || self.quiz_files.len() > 1 {
            egui::TopBottomPanel::bottom("navigation_footer").show_inside(ui, |ui| {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    // Bouton "Précédent" seulement si pas la première question
                    if self.current_question_index > 0 {
                        if ui.button("◀ Précédent").clicked() {
                            self.current_question_index -= 1;
                            self.selected_answer = None;
                        }
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Bouton "Suivant" ou "Terminer" selon la position
                        if self.current_question_index < self.questions.len() - 1 {
                            if ui.button("Suivant ▶").clicked() {
                                self.current_question_index += 1;
                                self.selected_answer = None;
                            }
                        } else {
                            if ui.button("Terminer").clicked() {
                                self.show_results = true;
                            }
                        }
                    });
                });
                ui.add_space(5.0);
            });
        } else {
            // Si qu'une seule question, afficher juste le bouton Terminer au centre
            egui::TopBottomPanel::bottom("navigation_footer").show_inside(ui, |ui| {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                        if ui.button("Terminer").clicked() {
                            self.show_results = true;
                        }
                    });
                });
                ui.add_space(5.0);
            });
        }
    }
    
    fn show_results_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading("Résultats du Quiz");
        ui.add_space(20.0);
        
        let mut correct_count = 0;
        let total = self.questions.len();
        
        for (i, quiz) in self.questions.iter().enumerate() {
            if let Some(Some(user_answer)) = self.user_answers.get(i) {
                if user_answer == &quiz.correct_answer {
                    correct_count += 1;
                }
            }
        }
        
        ui.label(format!("Score: {}/{}", correct_count, total));
        ui.label(format!("Pourcentage: {:.1}%", (correct_count as f32 / total as f32) * 100.0));
        ui.add_space(30.0);
        
        ui.label("Détails:");
        ui.add_space(10.0);
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, quiz) in self.questions.iter().enumerate() {
                let user_answer = self.user_answers.get(i).and_then(|a| a.as_ref());
                let is_correct = user_answer.map_or(false, |a| a == &quiz.correct_answer);
                
                ui.horizontal(|ui| {
                    ui.label(format!("Q{}: ", i + 1));
                    if is_correct {
                        ui.colored_label(egui::Color32::GREEN, "[✓] Correct");
                    } else {
                        ui.colored_label(egui::Color32::RED, "[✗] Incorrect");
                        if let Some(ans) = user_answer {
                            ui.label(format!("(Votre réponse: {}, Correcte: {})", ans, quiz.correct_answer));
                        } else {
                            ui.label(format!("(Non répondu, Correcte: {})", quiz.correct_answer));
                        }
                    }
                });
                ui.add_space(5.0);
            }
        });
        
        // Footer pour le bouton "Choisir un autre quiz"
        if self.quiz_files.len() >= 1 {
            egui::TopBottomPanel::bottom("results_footer").show_inside(ui, |ui| {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    if ui.button("◀ Choisir un autre quiz").clicked() {
                        self.show_quiz_selection = true;
                        self.show_results = false;
                    }
                });
                ui.add_space(5.0);
            });
        }
    }
    
    fn show_quiz_selection_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading("Sélection du Quiz");
        ui.add_space(20.0);
        
        ui.label("Choisissez un quiz :");
        ui.add_space(15.0);
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            for quiz_file in &self.quiz_files.clone() {
                let display_name = quiz_file.trim_end_matches(".txt").trim_end_matches(".yaml");
                if ui.button(display_name).clicked() {
                    self.load_selected_quiz(quiz_file);
                }
                ui.add_space(5.0);
            }
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_quiz_selection {
                self.show_quiz_selection_screen(ui);
            } else if self.show_results {
                self.show_results_screen(ui);
            } else {
                self.show_quiz_screen(ui);
            }
        });
    }
}
