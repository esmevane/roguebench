//! HTML template helpers
//!
//! Functional but not fancy - simple HTML generation.

/// Wrap content in a basic HTML page
pub fn page(title: &str, content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <style>
        body {{
            font-family: system-ui, -apple-system, sans-serif;
            max-width: 800px;
            margin: 2rem auto;
            padding: 0 1rem;
            line-height: 1.6;
        }}
        nav ul {{
            list-style: none;
            padding: 0;
            display: flex;
            gap: 1rem;
        }}
        nav a {{
            text-decoration: none;
            padding: 0.5rem 1rem;
            background: #eee;
            border-radius: 4px;
        }}
        nav a:hover {{
            background: #ddd;
        }}
        form {{
            display: flex;
            flex-direction: column;
            gap: 1rem;
            max-width: 400px;
        }}
        label {{
            display: flex;
            flex-direction: column;
            gap: 0.25rem;
        }}
        input[type="text"], input[type="number"] {{
            padding: 0.5rem;
            border: 1px solid #ccc;
            border-radius: 4px;
            font-size: 1rem;
        }}
        button {{
            padding: 0.5rem 1rem;
            background: #007bff;
            color: white;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-size: 1rem;
        }}
        button:hover {{
            background: #0056b3;
        }}
        button.danger {{
            background: #dc3545;
        }}
        button.danger:hover {{
            background: #c82333;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 1rem 0;
        }}
        th, td {{
            padding: 0.5rem;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }}
        th {{
            background: #f5f5f5;
        }}
        .actions {{
            display: flex;
            gap: 0.5rem;
        }}
        .actions a, .actions button {{
            padding: 0.25rem 0.5rem;
            font-size: 0.875rem;
        }}
        .message {{
            padding: 1rem;
            margin: 1rem 0;
            border-radius: 4px;
        }}
        .message.success {{
            background: #d4edda;
            color: #155724;
        }}
        .message.error {{
            background: #f8d7da;
            color: #721c24;
        }}
    </style>
</head>
<body>
    {content}
</body>
</html>"#
    )
}

/// Navigation bar
pub fn nav() -> &'static str {
    r#"
    <nav>
        <ul>
            <li><a href="/">Home</a></li>
            <li><a href="/enemies">Enemies</a></li>
        </ul>
    </nav>
    "#
}
