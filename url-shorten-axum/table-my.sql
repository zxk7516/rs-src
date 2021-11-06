CREATE TABLE short_links (
    id INT PRIMARY KEY auto_increment,
    url VARCHAR (255),
    UNIQUE KEY unique_url(`url`)
);