-- Your SQL goes here
CREATE TABLE contacts (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  cpf VARCHAR NOT NULL,
  age INTEGER DEFAULT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO contacts (name, cpf, age, published) VALUES
('Manorina melanotis', '14785236978', 18, FALSE),
('Dasyornis brachypterus', '14785236978', 18, FALSE),
('Swift Parrot', '14785236978', 18, FALSE),
('Australasian Bittern', '14785236978', 18, FALSE),
('Southern Giant Petrel', '14785236978', 18, FALSE),
('Neochmia ruficauda ruficauda', '14785236978', 18, FALSE),
('Gould s Petrel', '14785236978', 18, FALSE),
('Mallee Emu-wren', '14785236978', 18, FALSE),
('Coxens Fig-Parrot', '14785236978', 18, FALSE),
('Poephila cincta cincta', '14785236978', 18, FALSE),
('Chatham Albatross', '14785236978', 18, FALSE),
('Amytornis barbatus barbatus', '14785236978', 18, FALSE),
('Rostratula australis', '14785236978', 18, FALSE),
('Amsterdam Albatross', '14785236978', 18, FALSE),
('Northern Royal Albatross', '14785236978', 18, FALSE),
('Tristan Albatross', '14785236978', 18, FALSE);
