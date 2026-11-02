import type { QuestionDto } from './bindings/QuestionDto';
import type { AnswerDto } from './bindings/AnswerDto';
export type Question = QuestionDto;
export type Answer = AnswerDto;
export type Fields = Pick<Question, 'title' | 'name' | 'content'>;
export class ApiError extends Error {
  constructor(public status: number, public messages: string[]) { super(messages.join('\n')); }
}
export async function api<T>(path: string, method = 'GET', data?: unknown): Promise<T> {
  let response: Response;
  try {
    response = await fetch(`/api${path}`, { method, headers: data === undefined ? {} : { 'Content-Type': 'application/json' }, body: data === undefined ? undefined : JSON.stringify(data) });
  } catch { throw new ApiError(0, ['通信に失敗しました。接続を確認して再度お試しください。']); }
  if (!response.ok) {
    const body = await response.json().catch(() => ({}));
    const validation = body.errors && typeof body.errors === 'object'
      ? Object.entries(body.errors as Record<string, { message?: string; code?: string }[]>).flatMap(
        ([field, errors]) => errors.map(error => error.message ?? `${field}: ${error.code ?? 'invalid'}`),
      ) : [];
    const fallback = response.status === 404 ? '対象のデータが見つかりません。'
      : body.description ?? `処理に失敗しました (${response.status})。`;
    throw new ApiError(response.status, validation.length ? validation : [fallback]);
  }
  return response.status === 204 ? undefined as T : response.json();
}
export function messages(error: unknown): string[] {
  return error instanceof ApiError ? error.messages : ['処理に失敗しました。再度お試しください。'];
}
