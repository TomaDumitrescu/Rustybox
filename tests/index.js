"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.setCheckRunOutput = void 0;
var core = require("@actions/core");
var github = require("@actions/github");
var setCheckRunOutput = function (text) { return __awaiter(void 0, void 0, void 0, function () {
    var token, octokit, nwo, _a, owner, repo, runId, workflowRunResponse, checkSuiteUrl, checkSuiteId, checkRunsResponse, checkRun;
    return __generator(this, function (_b) {
        switch (_b.label) {
            case 0:
                // If we have nothing to output, then bail
                if (text === '')
                    return [2 /*return*/];
                token = process.env['GITHUB_TOKEN'] || core.getInput('token');
                if (!token || token === '')
                    return [2 /*return*/];
                octokit = github.getOctokit(token);
                if (!octokit)
                    return [2 /*return*/];
                nwo = process.env['GITHUB_REPOSITORY'] || '/';
                _a = nwo.split('/'), owner = _a[0], repo = _a[1];
                if (!owner)
                    return [2 /*return*/];
                if (!repo)
                    return [2 /*return*/];
                runId = parseInt(process.env['GITHUB_RUN_ID'] || '');
                if (Number.isNaN(runId))
                    return [2 /*return*/];
                return [4 /*yield*/, octokit.rest.actions.getWorkflowRun({
                        owner: owner,
                        repo: repo,
                        run_id: runId,
                    })
                    // Find the check suite run
                    // eslint-disable-next-line @typescript-eslint/no-explicit-any
                ];
            case 1:
                workflowRunResponse = _b.sent();
                checkSuiteUrl = workflowRunResponse.data.check_suite_url;
                checkSuiteId = parseInt(checkSuiteUrl.match(/[0-9]+$/)[0], 10);
                return [4 /*yield*/, octokit.rest.checks.listForSuite({
                        owner: owner,
                        repo: repo,
                        check_name: 'Autograding',
                        check_suite_id: checkSuiteId,
                    })];
            case 2:
                checkRunsResponse = _b.sent();
                checkRun = checkRunsResponse.data.total_count === 1 && checkRunsResponse.data.check_runs[0];
                if (!checkRun)
                    return [2 /*return*/];
                // Update the checkrun, we'll assign the title, summary and text even though we expect
                // the title and summary to be overwritten by GitHub Actions (they are required in this call)
                // We'll also store the total in an annotation to future-proof
                return [4 /*yield*/, octokit.rest.checks.update({
                        owner: owner,
                        repo: repo,
                        check_run_id: checkRun.id,
                        output: {
                            title: 'Autograding',
                            summary: text,
                            text: text,
                            annotations: [
                                {
                                    // Using the `.github` path is what GitHub Actions does
                                    path: '.github',
                                    start_line: 1,
                                    end_line: 1,
                                    annotation_level: 'notice',
                                    message: text,
                                    title: 'Autograding complete',
                                },
                            ],
                        },
                    })];
            case 3:
                // Update the checkrun, we'll assign the title, summary and text even though we expect
                // the title and summary to be overwritten by GitHub Actions (they are required in this call)
                // We'll also store the total in an annotation to future-proof
                _b.sent();
                return [2 /*return*/];
        }
    });
}); };
exports.setCheckRunOutput = setCheckRunOutput;
var points = process.argv[2];
core.setOutput('Points', points);
(0, exports.setCheckRunOutput)('Points ' + points);
