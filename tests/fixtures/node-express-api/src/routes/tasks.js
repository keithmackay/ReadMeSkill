const express = require('express');
const router = express.Router();

router.get('/', (req, res) => res.json([]));
router.post('/', (req, res) => res.status(201).json(req.body));
router.get('/:id', (req, res) => res.json({ id: req.params.id }));
router.put('/:id', (req, res) => res.json(req.body));
router.delete('/:id', (req, res) => res.status(204).send());

module.exports = router;
