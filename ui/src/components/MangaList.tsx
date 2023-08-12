import React from 'react';

interface Manga {
  id: string;
  title: string;
  // Add more properties here as needed
}

interface MangaListProps {
  mangas: Manga[];
}

const MangaList: React.FC<MangaListProps> = ({ mangas }) => {
  return (
    <div className="manga-list">
      <ul>
        {mangas.map((manga) => (
          <li key={manga.id}>{manga.title}</li>
        ))}
      </ul>
    </div>
  );
};

export default MangaList;

