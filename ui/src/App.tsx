import React, { useState, useEffect } from 'react';
import MangaList from './components/MangaList';
import MangaViewer from './components/MangaViewer';

interface Manga {
  id: string;
  title: string;
  // Add more properties here as needed
}

const App: React.FC = () => {
  const [mangas, setMangas] = useState<Manga[]>([]);
  const [selectedManga, setSelectedManga] = useState<Manga | null>(null);

  useEffect(() => {
    // Fetch manga list from your backend and update the state
    // For simplicity, mock data is used here
    const mockMangas: Manga[] = [
      { id: '1', title: 'Manga 1' },
      { id: '2', title: 'Manga 2' },
      // Add more mock data as needed
    ];
    setMangas(mockMangas);
  }, []);

  const handleMangaSelect = (manga: Manga) => {
    setSelectedManga(manga);
  };

  return (
    <div className="app">
      <h1>Manga App</h1>
      <MangaList mangas={mangas} />
      {selectedManga && (
        <MangaViewer mangaId={selectedManga.id} filename="page-1.jpg" />
      )}
    </div>
  );
};

export default App;

