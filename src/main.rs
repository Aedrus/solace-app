slint::include_modules!();

fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    // Utility Components
    component container inherits Rectangle {
    }
    component column inherits VerticalLayout {
    }
    component row inherits HorizontalLayout {
    }
    // Main Section Components
    component left-sidebar-slim inherits Rectangle {
        width: 48px;
        height: 100%;
        background: red;
    }
    component left-sidebar-explorer inherits Rectangle {
        width: 270px;
        height: 100%;
        background: #FBFDFF;
    }

    // UI Components
    component icon-btn inherits Rectangle {
        Text {text: "Icon"; color: black;}
        width: 32px;
        height: 32px;
        background: rgb(32,80,100);
        border-radius: 8px;
    }
    // Document/Window
    export component MainWindow inherits Window {
        // Sidebar Widget
        row {
            alignment: start;
            // Slim Sidebar (Far Left)
            left-sidebar-slim {
                column {
                    alignment: space-between;
                    row {
                        alignment: center;
                        padding-top: 16px;
                        icon-btn {}
                    }
                    row {
                        alignment: center;
                        padding-bottom: 16px;
                        column {
                            spacing: 12px;
                            icon-btn {}
                            icon-btn {}
                            icon-btn {}
                        }
                    }
                }
            }
            left-sidebar-explorer {
                column {
                    container {
                        border-color: rgba(0,0,0, 0.05);
                        border-width: 1px;
                        height: 64px;
                        row {
                            alignment: start;
                            column {
                                padding-left: 16px;
                                alignment: center;
                                Text {text: "Home"; color: black;}
                            }
                        }
                    }
                    Text {text: "Home";}
                }
            }
        }

        // Main Page
    }
}
