'use client';

import { type EditParams, editMemory } from '@actions/edit';
import type { Memory } from '@actions/timeline';
import { timestampDate } from '@bufbuild/protobuf/wkt';

import { Modal } from '@components';
import useToast from '@hooks/useToast';
import { AlertTriangle } from 'lucide-react';
import { useEffect, useState, useTransition } from 'react';

function toDateTimeLocal(date: Date): string {
	const pad = (n: number) => n.toString().padStart(2, '0');
	return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}T${pad(date.getHours())}:${pad(date.getMinutes())}`;
}

interface EditMemoryModalProps {
	memory: Memory;
	open: boolean;
	onClose: () => void;
	onMemoryUpdated: (updated: Memory) => void;
}

export default function EditMemoryModal({ memory, open, onClose, onMemoryUpdated }: EditMemoryModalProps) {
	const [content, setContent] = useState(memory.content);
	const [eventAt, setEventAt] = useState(memory.eventAt ? toDateTimeLocal(timestampDate(memory.eventAt)) : '');
	const [confirmed, setConfirmed] = useState(false);
	const [isPending, startTransition] = useTransition();
	const { success, error: showError } = useToast();

	useEffect(() => {
		if (open) {
			setContent(memory.content);
			setEventAt(memory.eventAt ? toDateTimeLocal(timestampDate(memory.eventAt)) : '');
			setConfirmed(false);
		}
	}, [open, memory]);

	function submit() {
		const params: EditParams = { pid: memory.pid };
		if (content !== memory.content) params.content = content;
		if (eventAt) {
			const parsed = new Date(eventAt);
			if (Number.isNaN(parsed.getTime())) {
				showError('Invalid event date');
				return;
			}
			params.eventAt = parsed;
		}
		if (params.content === undefined && params.eventAt === undefined) {
			onClose();
			return;
		}
		startTransition(async () => {
			const result = await editMemory(params);
			if (!result.success) {
				showError(result.error);
				return;
			}
			success('Memory updated');
			onMemoryUpdated(result.data.memory);
			onClose();
		});
	}

	return (
		<Modal open={open}>
			<div className="modal-box max-w-2xl">
				<h3 className="font-bold text-lg">Edit memory</h3>

				<div role="alert" className="alert alert-warning my-4 text-sm">
					<AlertTriangle className="h-5 w-5 shrink-0" />
					<span>
						Edit overwrites this memory permanently. The previous content cannot be recovered. To preserve history,
						supersede with a new memory instead.
					</span>
				</div>

				<form
					id="edit-memory-form"
					className="space-y-4"
					onSubmit={(e) => {
						e.preventDefault();
						submit();
					}}>
					<div>
						<label htmlFor="edit-memory-content" className="label">
							<span className="label-text">Content</span>
						</label>
						<textarea
							id="edit-memory-content"
							className="textarea textarea-bordered w-full"
							rows={4}
							value={content}
							disabled={isPending}
							onChange={(e) => setContent(e.target.value)}
						/>
					</div>

					<div>
						<label htmlFor="edit-memory-event-at" className="label">
							<span className="label-text">Event time (optional)</span>
						</label>
						<input
							id="edit-memory-event-at"
							type="datetime-local"
							className="input input-bordered"
							value={eventAt}
							disabled={isPending}
							onChange={(e) => setEventAt(e.target.value)}
						/>
					</div>

					<label htmlFor="edit-memory-confirm" className="label cursor-pointer justify-start gap-2">
						<input
							id="edit-memory-confirm"
							type="checkbox"
							className="checkbox checkbox-sm checkbox-warning"
							checked={confirmed}
							disabled={isPending}
							onChange={(e) => setConfirmed(e.target.checked)}
						/>
						<span className="label-text">I understand this overwrites the original permanently.</span>
					</label>

					<div className="modal-action">
						<button type="button" className="btn btn-ghost" disabled={isPending} onClick={onClose}>
							Cancel
						</button>
						<button type="submit" className="btn btn-warning" disabled={!confirmed || isPending}>
							{isPending ? (
								<>
									<span className="loading loading-spinner loading-sm" />
									Saving...
								</>
							) : (
								'Overwrite memory'
							)}
						</button>
					</div>
				</form>
			</div>
		</Modal>
	);
}
