import React from 'react';

interface MangaViewerProps {
  mangaId: string;
  filename: string;
}

const MangaViewer: React.FC<MangaViewerProps> = ({ mangaId, filename }) => {
  const imageUrl = `/media/${mangaId}/${filename}`;

  return (
    <div className="manga-viewer">
      <img src={imageUrl} alt="Manga Page" />
    </div>
  );
};

export default MangaViewer;

