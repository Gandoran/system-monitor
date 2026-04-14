import { useState } from "react";
import { SessionResults } from "../../types/session";
import { SessionCard } from "../cards/SessionCard";
import { C, Card } from "../ui/SharedUi";
import { SessionHeader } from "./session/SessionHeader";
import { SessionPagination } from "./session/SessionPagination";

interface SessionTabProps {
  isRunning: boolean;
  history: SessionResults[];
  startSession: () => void;
  stopSession: () => void;
  deleteSession: (index: number) => void; 
}

export function SessionTab({ isRunning, history, startSession, stopSession, deleteSession }: SessionTabProps) {
  const [currentPage, setCurrentPage] = useState(0);
  const itemsPerPage = 5;
  const totalPages = Math.ceil(history.length / itemsPerPage);
  const startIndex = currentPage * itemsPerPage;
  const currentSessions = history.slice(startIndex, startIndex + itemsPerPage);
  const hasNextPage = currentPage < totalPages - 1;
  const hasPrevPage = currentPage > 0;

  return (
    <div style={{ padding: "10px", fontFamily: "monospace" }}>
      <SessionHeader isRunning={isRunning} onStart={startSession} onStop={stopSession} />
      {isRunning && (
        <Card accent={C.ram} style={{ padding: 12, marginBottom: 24 }}>
          <span style={{ color: C.text, fontSize: 13 }}>🔴 Registration in progress...</span>
        </Card>
      )}
      <div style={{ display: "flex", flexDirection: "column", gap: 16 }}>
        {history.length === 0 && !isRunning && (
          <div style={{ textAlign: "center", color: C.muted, padding: 40, border: `1px dashed ${C.border}`, borderRadius: 12 }}>
            No benchmark registered. Press Start for a new one!
          </div>
        )}
        {currentSessions.map((session, index) => {
          const realIndex = startIndex + index; 
          const absoluteIndex = history.length - realIndex; 
          return (
            <SessionCard key={realIndex} s={session}  i={realIndex} absoluteIndex={absoluteIndex} onDelete={deleteSession}/>
          );
        })}
      </div>
      {history.length > itemsPerPage && (
        <SessionPagination currentPage={currentPage}totalPages={totalPages} hasPrevPage={hasPrevPage} hasNextPage={hasNextPage} onPrev={() => setCurrentPage(prev => prev - 1)} onNext={() => setCurrentPage(prev => prev + 1)}/>
      )}
    </div>
  );
}