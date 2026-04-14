import { C } from "../../ui/SharedUi";

interface SessionPaginationProps {
  currentPage: number;
  totalPages: number;
  hasPrevPage: boolean;
  hasNextPage: boolean;
  onPrev: () => void;
  onNext: () => void;
}

export function SessionPagination({ currentPage, totalPages, hasPrevPage, hasNextPage, onPrev, onNext }: SessionPaginationProps) {
  return (
    <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginTop: 24 }}>
      <button disabled={!hasPrevPage} onClick={onPrev}
        style={{ 
          padding: "6px 12px", background: hasPrevPage ? C.surface : "transparent", 
          color: hasPrevPage ? C.text : C.muted, border: `1px solid ${hasPrevPage ? C.border : "transparent"}`, 
          borderRadius: 6, cursor: hasPrevPage ? "pointer" : "default", opacity: hasPrevPage ? 1 : 0.3,
          transition: "all 0.2s"
        }}>← Recent</button>
      <span style={{ fontSize: 12, color: C.muted }}>
        Page {currentPage + 1} of {totalPages}
      </span> 
      <button disabled={!hasNextPage} onClick={onNext}
        style={{ 
          padding: "6px 12px", background: hasNextPage ? C.surface : "transparent", 
          color: hasNextPage ? C.text : C.muted, border: `1px solid ${hasNextPage ? C.border : "transparent"}`, 
          borderRadius: 6, cursor: hasNextPage ? "pointer" : "default", opacity: hasNextPage ? 1 : 0.3, transition: "all 0.2s"
        }}>Old →
      </button>
    </div>
  );
}