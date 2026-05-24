CREATE TABLE Autores (
    id_autor SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    nacionalidad VARCHAR(50)
);

CREATE TABLE Editoriales (
    id_editorial SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    pais VARCHAR(50)
);

CREATE TABLE Libros (
    id_libro SERIAL PRIMARY KEY,
    titulo VARCHAR(150) NOT NULL,
    isbn VARCHAR(20) UNIQUE,
    anio_publicacion INT,
    id_autor INT REFERENCES Autores(id_autor),
    id_editorial INT REFERENCES Editoriales(id_editorial)
);

CREATE TABLE Usuarios (
    id_usuario SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    direccion TEXT,
    fecha_registro DATE DEFAULT CURRENT_DATE
);

CREATE TABLE Prestamos (
    id_prestamo SERIAL PRIMARY KEY,
    id_libro INT REFERENCES Libros(id_libro),
    id_usuario INT REFERENCES Usuarios(id_usuario),
    fecha_salida DATE DEFAULT CURRENT_DATE,
    fecha_devolucion DATE
);